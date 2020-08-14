use crate::*;

pub mod bars;
mod export;
pub mod logs;
pub mod time;

use bars::Bars;
use logs::Logs;
use repo::YewRepo;

pub struct Root {
    mode: Mode,
    show_bars: Option<Callback<()>>,
    show_logs: Option<Callback<()>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Bars,
    Logs,
}

pub struct SwitchModeMsg(Mode);

impl Component for Root {
    type Message = SwitchModeMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show_bars = link.callback(|()| SwitchModeMsg(Mode::Bars));
        let show_logs = link.callback(|()| SwitchModeMsg(Mode::Logs));
        Self {
            mode: Mode::Bars,
            show_bars: Some(show_bars),
            show_logs: Some(show_logs),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let old = self.mode;
        self.mode = msg.0;
        self.mode != old
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
                <Bars show_logs={self.show_logs.as_ref().expect("logs_cb")} />
            },
            Mode::Logs => html! {
                <Logs show_bars={self.show_bars.as_ref().expect("bars_cb")} />
            },
        }
    }
}

#[derive(Clone, Serialize, Debug)]
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

impl StorageState {
    fn reload_text(&mut self, repo: &YewRepo, text_type: TextType) {
        match text_type {
            TextType::Meds => self.meds = repo.load_text(text_type).unwrap_or_default(),
            TextType::Sleep => self.sleep_entries = repo.load_text(text_type).unwrap_or_default(),
            TextType::Notes => self.notes = repo.load_text(text_type).unwrap_or_default(),
        }
    }

    fn reload_moods(&mut self, repo: &YewRepo) {
        self.mood_readings = repo.load_mood_readings().unwrap_or_default()
    }
}
