use std::collections::HashMap;

use chrono::NaiveDate;

use super::about;
use super::time::formatted_js_date;
use super::StorageState;
use crate::pips::{blue_circles, red_circles};
use crate::*;

pub struct Logs {
    entries: Vec<(NaiveDate, Vec<Entry>)>,
    mode: LogsMode,
}

pub enum LogsMsg {
    ShowHome,
    ShowBackdate,
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
    pub show_home: Callback<()>,
    pub show_backdate: Callback<()>,
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

impl Component for Logs {
    type Message = LogsMsg;
    type Properties = LogsProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let entries = derive_entries(&ctx.props().storage_state);

        let mode = LogsMode::View;

        Self { entries, mode }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LogsMsg::ShowBackdate => {
                ctx.props().show_backdate.emit(());
                false
            }
            LogsMsg::ShowHome => {
                ctx.props().show_home.emit(());
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
                ctx.props().replace_mood_readings.emit(
                    self.entries
                        .iter()
                        .map(|(_, entries)| entries)
                        .flatten()
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
                ctx.props().replace_texts.emit((
                    TextType::Meds,
                    self.entries
                        .iter()
                        .map(|(_, entries)| entries)
                        .flatten()
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

                ctx.props().replace_texts.emit((
                    TextType::Notes,
                    self.entries
                        .iter()
                        .map(|(_, entries)| entries)
                        .flatten()
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
                ctx.props().replace_texts.emit((
                    TextType::Sleep,
                    self.entries
                        .iter()
                        .map(|(_, entries)| entries)
                        .flatten()
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

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.storage_state != ctx.props().storage_state {
            self.entries = derive_entries(&ctx.props().storage_state);
            true
        } else {
            false
        }
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        if self.mode == LogsMode::About {
            let callback = ctx.link().callback(|_| LogsMsg::ToggleAboutMode);
            about::section(callback, ctx)
        } else {
            html! { <>
                <div id="logs-button-grid">
                    <div class="center">
                        <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| LogsMsg::ToggleAboutMode)}>{ "About 🤔" }</button>
                    </div>
                    <div class="center">
                        <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| LogsMsg::ToggleDeleteMode )}>{ "Delete 🗑" }</button>
                    </div>
                    <div class="center">
                        <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| LogsMsg::ShowBackdate)}>{ "Backdate 🗓️" }</button>
                    </div>
                    <div class="center">
                        <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| LogsMsg::ShowHome)}>{ "Home 🔵🔴" }</button>
                    </div>
                </div>
                <div id="log-entries">
                    { self.entries.iter().map(|(date, entries)| self.render_entries(ctx, date, entries.clone(),  self.mode)).collect::<Html>() }
                </div>
            </> }
        }
    }
}

impl Logs {
    fn render_entries(
        &self,
        ctx: &yew::Context<Self>,
        date: &NaiveDate,
        entries: Vec<Entry>,
        logs_mode: LogsMode,
    ) -> Html {
        // Format date as "Monday, January 1st 2023"
        let date_string: String = date.format("%A, %B %-d, %Y").to_string();
        html! {
            <>
                <div class="date">{ date_string }</div>
                { entries.iter().map(|e| self.render_entry(ctx, e.clone(), logs_mode)).collect::<Html>() }
            </>
        }
    }
    fn render_entry(&self, ctx: &yew::Context<Self>, e: Entry, logs_mode: LogsMode) -> Html {
        let date_string: String = formatted_js_date(e.timestamp());
        match e {
            Entry::Mood(MoodReading {
                value,
                epoch_millis,
            }) => html! {
                <div>
                    { format!("{} {}", date_string,
                        if value == 0 { "⚪".to_string() }
                        else {
                            if value > 0 {
                                red_circles(value).replace("⚫", "")
                            } else {
                                blue_circles(value).replace("⚫", "")
                                 .chars().rev().collect()
                            }}) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button class="fancy-button" role="button" onclick={ctx.link().callback(move |_| LogsMsg::Delete(Entry::Mood(MoodReading {
                                value,
                                epoch_millis,
                            })))}>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </div>
            },
            Entry::Sleep(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <div>
                    { format!("{} 😴 {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button class="fancy-button" role="button" onclick={ctx.link().callback(move |_| LogsMsg::Delete(Entry::Sleep(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))}>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </div>
            },
            Entry::Meds(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <div>
                    { format!("{} 💊 {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button class="fancy-button" role="button"  onclick={ctx.link().callback(move |_| LogsMsg::Delete(Entry::Meds(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))}>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </div>
            },
            Entry::Note(TextSubmission {
                value,
                epoch_millis,
            }) => html! {
                <div>
                    { format!("{} 🗒️ {}", date_string, value) }
                    {
                        match logs_mode {
                            LogsMode::Delete => html! { <button class="fancy-button" role="button"  onclick={ctx.link().callback(move |_| LogsMsg::Delete(Entry::Note(TextSubmission {
                                value: value.clone(),
                                epoch_millis,
                            })))}>{ "DELETE" }</button> },
                            _ => html! { }
                        }
                    }
                </div>
            },
        }
    }

    fn delete_entry(&mut self, entry: Entry) {
        unimplemented!() //self.entries.retain(|e| e != &entry)
    }
}

fn derive_entries(storage_state: &StorageState) -> Vec<(NaiveDate, Vec<Entry>)> {
    let mut entries: HashMap<NaiveDate, Vec<Entry>> = HashMap::new();
    for m in &storage_state.mood_readings {
        if let Some(e) = entries.get_mut(&entry_date(m)) {
            e.push(Entry::Mood(m.clone()))
        } else {
            entries.insert(entry_date(m), vec![Entry::Mood(m.clone())]);
        }
    }
    for s in &storage_state.sleep_entries {
        if let Some(e) = entries.get_mut(&entry_date_text(s)) {
            e.push(Entry::Sleep(s.clone()))
        } else {
            entries.insert(entry_date_text(s), vec![Entry::Sleep(s.clone())]);
        }
    }
    for m in &storage_state.meds {
        if let Some(e) = entries.get_mut(&entry_date_text(m)) {
            e.push(Entry::Meds(m.clone()))
        } else {
            entries.insert(entry_date_text(m), vec![Entry::Meds(m.clone())]);
        }
    }
    for n in &storage_state.notes {
        if let Some(e) = entries.get_mut(&entry_date_text(n)) {
            e.push(Entry::Note(n.clone()))
        } else {
            entries.insert(entry_date_text(n), vec![Entry::Note(n.clone())]);
        }
    }

    let mut out: Vec<(NaiveDate, Vec<Entry>)> = entries.into_iter().collect();
    out.sort_by(|a, b| b.0.cmp(&a.0));
    out
}
fn entry_date(mr: &MoodReading) -> NaiveDate {
    let date = js_sys::Date::new(&JsValue::from_f64(mr.epoch_millis as f64));

    NaiveDate::from_ymd_opt(
        date.get_utc_full_year() as i32,
        date.get_utc_month() as u32 + 1,
        date.get_utc_date() as u32,
    )
    .unwrap()
}
fn entry_date_text(text: &TextSubmission) -> NaiveDate {
    let date = js_sys::Date::new(&JsValue::from_f64(text.epoch_millis as f64));

    NaiveDate::from_ymd_opt(
        date.get_utc_full_year() as i32,
        date.get_utc_month() as u32 + 1,
        date.get_utc_date() as u32,
    )
    .unwrap()
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
