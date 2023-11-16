mod about;
pub mod bars;
mod history;
pub mod logs;
pub mod time;

use crate::*;
use bars::Bars;
use history::History;
use logs::Logs;
use repo::YewRepo;

pub struct Root {
    mode: Mode,
    repo: YewRepo,
    storage_state: StorageState,
    show_bars: Option<Callback<()>>,
    show_logs: Option<Callback<()>>,
    add_mood_reading: Option<Callback<MoodReading>>,
    add_text: Option<Callback<(TextType, String)>>,
    replace_texts: Option<Callback<(TextType, Vec<TextSubmission>)>>,
    replace_mood_readings: Option<Callback<Vec<MoodReading>>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Bars,
    Logs,
    History,
}

pub enum RootMsg {
    SwitchMode(Mode),
    AddMoodReading(MoodReading),
    AddText(TextType, String),
    ReplaceMoodReadings(Vec<MoodReading>),
    ReplaceTexts(TextType, Vec<TextSubmission>),
}

impl Component for Root {
    type Message = RootMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show_bars = Some(link.callback(|()| RootMsg::SwitchMode(Mode::Bars)));
        let show_logs = Some(link.callback(|()| RootMsg::SwitchMode(Mode::Logs)));

        let show_history = Some(link.callback(|()| RootMsg::SwitchMode(Mode::History)));
        let add_text = Some(link.callback(|(text_type, text)| RootMsg::AddText(text_type, text)));
        let add_mood_reading =
            Some(link.callback(|mood_reading| RootMsg::AddMoodReading(mood_reading)));
        let replace_texts =
            Some(link.callback(|(text_type, text)| RootMsg::ReplaceTexts(text_type, text)));
        let replace_mood_readings =
            Some(link.callback(|readings| RootMsg::ReplaceMoodReadings(readings)));

        let repo = YewRepo::new();
        let storage_state = StorageState::load(&repo);

        Self {
            mode: Mode::History,
            repo,
            storage_state,
            show_bars,
            show_logs,
            add_mood_reading,
            add_text,
            replace_texts,
            replace_mood_readings,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMsg::SwitchMode(new_mode) => {
                let old = self.mode;
                self.mode = new_mode;
                self.mode != old
            }
            RootMsg::AddText(TextType::Sleep, text) => {
                self.storage_state
                    .sleep_entries
                    .push(TextSubmission::new(text));

                self.repo
                    .save_text(TextType::Sleep, &self.storage_state.sleep_entries)
                    .expect("save sleep");

                true
            }
            RootMsg::AddText(TextType::Meds, text) => {
                self.storage_state.meds.push(TextSubmission::new(text));
                self.repo
                    .save_text(TextType::Meds, &self.storage_state.meds)
                    .expect("save meds");
                true
            }
            RootMsg::AddText(TextType::Notes, text) => {
                self.storage_state.notes.push(TextSubmission::new(text));

                self.repo
                    .save_text(TextType::Notes, &self.storage_state.notes)
                    .expect("save notes");
                true
            }
            RootMsg::AddMoodReading(value) => {
                self.storage_state.mood_readings.push(value);
                self.repo
                    .save_mood_readings(&self.storage_state.mood_readings)
                    .expect("save mood readings");
                true
            }
            RootMsg::ReplaceMoodReadings(readings) => {
                self.storage_state.mood_readings = readings.clone();
                self.repo
                    .save_mood_readings(&readings)
                    .expect("replace mood readings");
                true
            }
            RootMsg::ReplaceTexts(text_type, all) => {
                match text_type {
                    TextType::Meds => self.storage_state.meds = all.clone(),
                    TextType::Notes => self.storage_state.notes = all.clone(),
                    TextType::Sleep => self.storage_state.sleep_entries = all.clone(),
                };
                self.repo
                    .save_text(text_type, &all)
                    .expect("replace text entries");
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
        match self.mode {
            Mode::Bars => html! {
                <Bars
                    storage_state={self.storage_state.clone()}
                    show_logs={self.show_logs.as_ref().expect("logs_cb")}
                    add_mood_reading={self.add_mood_reading.as_ref().expect("smrcb")},
                    add_text={self.add_text.as_ref().expect("smtcb")}
                />
            },
            Mode::Logs => html! {
                <Logs
                    storage_state={self.storage_state.clone()}
                    show_bars={self.show_bars.as_ref().expect("bars_cb")}
                    replace_mood_readings={self.replace_mood_readings.as_ref().expect("rmr_cb")}
                    replace_texts={self.replace_texts.as_ref().expect("rt_cb")}
                />
            },
            Mode::History => html! {
                <History
                    storage_state={self.storage_state.clone()}
                    show_logs={self.show_logs.as_ref().expect("logs_cb")}
                    add_mood_reading={self.add_mood_reading.as_ref().expect("smrcb")},
                    add_text={self.add_text.as_ref().expect("smtcb")}
                />
            },
        }
    }
}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct StorageState {
    mood_readings: Vec<MoodReading>,
    meds: Vec<TextSubmission>,
    sleep_entries: Vec<TextSubmission>,
    notes: Vec<TextSubmission>,
}

impl StorageState {
    pub fn load(repo: &YewRepo) -> Self {
        Self {
            mood_readings: repo.load_mood_readings().unwrap_or_default(),
            meds: repo.load_text(TextType::Meds).unwrap_or_default(),
            sleep_entries: repo.load_text(TextType::Sleep).unwrap_or_default(),
            notes: repo.load_text(TextType::Notes).unwrap_or_default(),
        }
    }
}
