use super::StorageState;
use crate::*;
use repo::YewRepo;
use std::rc::Rc;
use web::time::js_local_datetime;

pub struct Logs {
    link: ComponentLink<Self>,
    entries: Vec<Entry>,
    storage_state: Rc<StorageState>,
    repo: Rc<YewRepo>,
    mode: LogsMode,
    show_bars: Callback<()>,
}

pub enum LogsMsg {
    ShowBars,
    ToggleDeleteMode,
    Delete(Entry),
}

#[derive(Copy, Clone)]
pub enum LogsMode {
    View,
    Delete,
    _Edit,
}

#[derive(Properties, Clone)]
pub struct LogsProps {
    pub show_bars: Callback<()>,
    pub repo: Rc<YewRepo>,
    pub storage_state: Rc<StorageState>,
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

impl Component for Logs {
    type Message = LogsMsg;
    type Properties = LogsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut entries = vec![];
        for m in &props.storage_state.mood_readings {
            entries.push(Entry::Mood(*m))
        }
        for s in &props.storage_state.sleep_entries {
            entries.push(Entry::Sleep(s.clone()))
        }
        for m in &props.storage_state.meds {
            entries.push(Entry::Meds(m.clone()))
        }
        for n in &props.storage_state.notes {
            entries.push(Entry::Note(n.clone()))
        }
        entries.sort();
        entries.reverse();

        let mode = LogsMode::View;

        Self {
            link,
            entries,
            storage_state: props.storage_state,
            repo: props.repo,
            mode,
            show_bars: props.show_bars,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LogsMsg::ShowBars => {
                self.show_bars.emit(());
                false
            }
            LogsMsg::ToggleDeleteMode => {
                self.mode = match self.mode {
                    LogsMode::Delete => LogsMode::View,
                    _ => LogsMode::Delete,
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
                todo!("lift");
                self.repo
                    .save_mood_readings(
                        &self
                            .entries
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
                    )
                    .expect("save");
                true
            }
            LogsMsg::Delete(Entry::Meds(m)) => {
                self.delete_entry(Entry::Meds(m));
                todo!("lift");
                self.repo
                    .save_text(
                        TextType::Meds,
                        &self
                            .entries
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
                    )
                    .expect("save");

                true
            }
            LogsMsg::Delete(Entry::Note(m)) => {
                self.delete_entry(Entry::Note(m));
                todo!("lift");
                self.repo
                    .save_text(
                        TextType::Notes,
                        &self
                            .entries
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
                    )
                    .expect("save");

                true
            }
            LogsMsg::Delete(Entry::Sleep(m)) => {
                self.delete_entry(Entry::Sleep(m));
                todo!("lift");
                self.repo
                    .save_text(
                        TextType::Sleep,
                        &self
                            .entries
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
                    )
                    .expect("save");

                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div id="logsbuttongrid">
                    <div class="center">
                        <button class="thick">{ "Update 🖊" }</button>
                    </div>
                    <div class="center">
                        <button class="thick" onclick=self.link.callback(|_| LogsMsg::ToggleDeleteMode )>{ "Delete 🗑" }</button>
                    </div>
                    <div class="center">
                        { super::export::button(&self.storage_state) }
                    </div>
                    <div class="center">
                        <button class="thick" onclick=self.link.callback(|_| LogsMsg::ShowBars)>{ "Bars 📊"}</button>
                    </div>
                </div>
                <ul>
                    { self.entries.iter().map(|e| self.render_entry(e.clone(), self.mode)).collect::<Html>() }
                </ul>
            </>
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
                            LogsMode::_Edit => html! { <button>{ "EDIT" }</button> },
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
                            LogsMode::_Edit => html! { <button>{ "EDIT" }</button> },
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
                            LogsMode::_Edit => html! { <button>{ "EDIT" }</button> },
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
                            LogsMode::_Edit => html! { <button>{ "EDIT" }</button> },
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
