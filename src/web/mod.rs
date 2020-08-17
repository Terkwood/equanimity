pub mod bars;
mod export;
pub mod logs;
pub mod time;

use crate::*;
use bars::Bars;
use logs::Logs;
use repo::YewRepo;

pub struct Root {
    mode: Mode,
    repo: YewRepo,
    storage_state: StorageState,
    show_bars: Option<Callback<()>>,
    show_logs: Option<Callback<()>>,
    submit_mood_reading: Option<Callback<MoodReading>>,
    submit_text: Option<Callback<(TextType, String)>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Bars,
    Logs,
}

pub enum RootMsg {
    SwitchMode(Mode),
    SubmitMoodReading(MoodReading),
    SubmitText(TextType, String),
}

impl Component for Root {
    type Message = RootMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show_bars = link.callback(|()| RootMsg::SwitchMode(Mode::Bars));
        let show_logs = link.callback(|()| RootMsg::SwitchMode(Mode::Logs));
        let submit_text = link.callback(|(text_type, text)| RootMsg::SubmitText(text_type, text));
        let submit_mood_reading =
            link.callback(|mood_reading| RootMsg::SubmitMoodReading(mood_reading));
        let repo = YewRepo::new();
        let storage_state = StorageState::load(&repo);

        Self {
            mode: Mode::Bars,
            repo,
            storage_state,
            show_bars: Some(show_bars),
            show_logs: Some(show_logs),
            submit_mood_reading: Some(submit_mood_reading),
            submit_text: Some(submit_text),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMsg::SwitchMode(new_mode) => {
                let old = self.mode;
                self.mode = new_mode;
                self.mode != old
            }
            RootMsg::SubmitText(TextType::Sleep, text) => {
                self.storage_state
                    .sleep_entries
                    .push(TextSubmission::new(text));

                self.repo
                    .save_text(TextType::Sleep, &self.storage_state.sleep_entries)
                    .expect("save sleep");

                true
            }
            RootMsg::SubmitText(TextType::Meds, text) => {
                self.storage_state.meds.push(TextSubmission::new(text));
                self.repo
                    .save_text(TextType::Meds, &self.storage_state.meds)
                    .expect("save meds");
                true
            }
            RootMsg::SubmitText(TextType::Notes, text) => {
                self.storage_state.notes.push(TextSubmission::new(text));

                self.repo
                    .save_text(TextType::Notes, &self.storage_state.notes)
                    .expect("save notes");
                true
            }
            RootMsg::SubmitMoodReading(value) => {
                self.storage_state.mood_readings.push(value);
                self.repo
                    .save_mood_readings(&self.storage_state.mood_readings)
                    .expect("save mood readings");
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
                    submit_mood_reading={self.submit_mood_reading.as_ref().expect("smrcb")},
                    submit_text={self.submit_text.as_ref().expect("smtcb")}
                />
            },
            Mode::Logs => html! {
                <Logs storage_state={self.storage_state.clone()} show_bars={self.show_bars.as_ref().expect("bars_cb")} />
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
