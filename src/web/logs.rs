use super::about;
use super::StorageState;
use crate::*;
use web::time::js_local_datetime;
use yew_export_button::{export_button, ButtonOpts};

pub struct Logs {
    link: ComponentLink<Self>,
    entries: Vec<Entry>,
    mode: LogsMode,
    props: LogsProps,
}

pub enum LogsMsg {
    ShowMain,
    ToggleDeleteMode,
    ToggleAboutMode,
    Delete(Entry),
}

#[derive(Copy, Clone, PartialEq)]
pub enum LogsMode {
    View,
    Delete,
    About,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LogsProps {
    pub show_bars: Callback<()>,
    pub storage_state: StorageState,
    pub replace_texts: Callback<(TextType, Vec<TextSubmission>)>,
    pub replace_mood_readings: Callback<Vec<MoodReading>>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Entry {
    Mood(MoodReading),
    Sleep(TextSubmission),
    Meds(TextSubmission),
    Note(TextSubmission),
}

impl Entry {
    pub fn timestamp(&self) -> u64 {
        match self {
            Entry::Mood(m) => m.epoch_millis,
            Entry::Sleep(t) => t.epoch_millis,
            Entry::Meds(m) => m.epoch_millis,
            Entry::Note(t) => t.epoch_millis,
        }
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}

const EXPORT_BUTTON_CSS_ID: &str = "exportbutton";
const EXPORT_LINK_CSS_CLASS: &str = "download";
const EXPORT_FILE_PREFIX: &str = "equanimity";

impl Component for Logs {
    type Message = LogsMsg;
    type Properties = LogsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = derive_entries(&props.storage_state);

        let mode = LogsMode::View;

        Self {
            link,
            entries,
            mode,
            props,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LogsMsg::ShowMain => {
                self.props.show_bars.emit(());
                false
            }
            LogsMsg::ToggleDeleteMode => {
                self.mode = match self.mode {
                    LogsMode::Delete => LogsMode::View,
                    _ => LogsMode::Delete,
                };
                true
            }
            LogsMsg::ToggleAboutMode => {
                self.mode = match self.mode {
                    LogsMode::About => LogsMode::View,
                    _ => LogsMode::About,
                };
                true
            }
            LogsMsg::Delete(Entry::Mood(MoodReading {
                epoch_millis,
                value,
            })) => {
                self.delete_entry(Entry::Mood(MoodReading {
                    epoch_millis,
                    value,
                }));
                self.props.replace_mood_readings.emit(
                    self.entries
                        .iter()
                        .filter_map(|e| match e {
                            Entry::Mood(MoodReading {
                                epoch_millis,
                                value,
                            }) => Some(MoodReading {
                                epoch_millis: *epoch_millis,
                                value: value.clone(),
                            }),
                            _ => None,
                        })
                        .collect(),
                );
                true
            }
            LogsMsg::Delete(Entry::Meds(m)) => {
                self.delete_entry(Entry::Meds(m));
                self.props.replace_texts.emit((
                    TextType::Meds,
                    self.entries
                        .iter()
                        .filter_map(|e| match e {
                            Entry::Meds(TextSubmission {
                                epoch_millis,
                                value,
                            }) => Some(TextSubmission {
                                epoch_millis: *epoch_millis,
                                value: value.clone(),
                            }),
                            _ => None,
                        })
                        .collect(),
                ));
                true
            }
            LogsMsg::Delete(Entry::Note(m)) => {
                self.delete_entry(Entry::Note(m));

                self.props.replace_texts.emit((
                    TextType::Notes,
                    self.entries
                        .iter()
                        .filter_map(|e| match e {
                            Entry::Note(TextSubmission {
                                epoch_millis,
                                value,
                            }) => Some(TextSubmission {
                                epoch_millis: *epoch_millis,
                                value: value.clone(),
                            }),
                            _ => None,
                        })
                        .collect(),
                ));
                true
            }
            LogsMsg::Delete(Entry::Sleep(m)) => {
                self.delete_entry(Entry::Sleep(m));
                self.props.replace_texts.emit((
                    TextType::Sleep,
                    self.entries
                        .iter()
                        .filter_map(|e| match e {
                            Entry::Sleep(TextSubmission {
                                epoch_millis,
                                value,
                            }) => Some(TextSubmission {
                                epoch_millis: *epoch_millis,
                                value: value.clone(),
                            }),
                            _ => None,
                        })
                        .collect(),
                ));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.entries = derive_entries(&self.props.storage_state);
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        if self.mode == LogsMode::About {
            let callback = self.link.callback(|_| LogsMsg::ToggleAboutMode);
            about::section(callback)
        } else {
            let export_button = export_button(
                &self.props.storage_state,
                ButtonOpts {
                    a_class: EXPORT_LINK_CSS_CLASS,
                    button_id: EXPORT_BUTTON_CSS_ID,
                    file_prefix: EXPORT_FILE_PREFIX,
                    utc_millis: utc_now(),
                },
            );
            html! { <>
                <div id="logsbuttongrid">
                    <div class="center">
                        <button class="thick" onclick=self.link.callback(|_| LogsMsg::ToggleAboutMode)>{ "About 🤔" }</button>
                    </div>
                    <div class="center">
                        <button class="thick" onclick=self.link.callback(|_| LogsMsg::ToggleDeleteMode )>{ "Delete 🗑" }</button>
                    </div>
                    <div class="center">
                        { export_button }
                    </div>
                    <div class="center">
                        <button class="thick" onclick=self.link.callback(|_| LogsMsg::ShowBars)>{ "Bars 📊"}</button>
                    </div>
                </div>
                <ul>
                    { self.entries.iter().map(|e| self.render_entry(e.clone(), self.mode)).collect::<Html>() }
                </ul>
            </> }
        }
    }
}

impl Logs {
    fn render_entry(&self, e: Entry, logs_mode: LogsMode) -> Html {
        let dt = js_local_datetime(e.timestamp());
        let date_string = dt.format("%m/%d %R").to_string();
        match e {
            Entry::Mood(MoodReading {
                value,
                epoch_millis,
            }) => html! {
                <li>
                    { format!("[{} mood] {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button onclick=self.link.callback(move |_| LogsMsg::Delete(Entry::Mood(MoodReading {
                                value,
                                epoch_millis,
                            })))>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </li>
            },
            Entry::Sleep(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <li>
                    { format!("[{} sleep] {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button onclick=self.link.callback(move |_| LogsMsg::Delete(Entry::Sleep(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </li>
            },
            Entry::Meds(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <li>
                    { format!("[{} meds] {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button onclick=self.link.callback(move |_| LogsMsg::Delete(Entry::Meds(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </li>
            },
            Entry::Note(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <li>
                    { format!("[{} note] {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button onclick=self.link.callback(move |_| LogsMsg::Delete(Entry::Note(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </li>
            },
        }
    }

    fn delete_entry(&mut self, entry: Entry) {
        self.entries.retain(|e| e != &entry)
    }
}

fn derive_entries(storage_state: &StorageState) -> Vec<Entry> {
    let mut entries = vec![];
    for m in &storage_state.mood_readings {
        entries.push(Entry::Mood(*m))
    }
    for s in &storage_state.sleep_entries {
        entries.push(Entry::Sleep(s.clone()))
    }
    for m in &storage_state.meds {
        entries.push(Entry::Meds(m.clone()))
    }
    for n in &storage_state.notes {
        entries.push(Entry::Note(n.clone()))
    }
    entries.sort();
    entries.reverse();

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_entries() {
        let should_be_first = Entry::Sleep(TextSubmission {
            value: "hello".to_string(),
            epoch_millis: 0,
        });

        let should_be_middle = Entry::Note(TextSubmission {
            value: "yes".to_string(),
            epoch_millis: 50,
        });

        let should_be_last = Entry::Mood(MoodReading {
            value: 0,
            epoch_millis: 100,
        });

        let mut entries = vec![
            should_be_last.clone(),
            should_be_first.clone(),
            should_be_middle.clone(),
        ];

        entries.sort();

        assert_eq!(
            entries,
            vec![should_be_first, should_be_middle, should_be_last]
        )
    }
}
