pub mod bars;
mod export;
pub mod logs;
pub mod time;

use crate::*;
use bars::Bars;
use logs::Logs;
use repo::YewRepo;
use std::rc::Rc;

pub struct Root {
    mode: Mode,
    repo: Rc<YewRepo>,
    storage_state: Rc<StorageState>,
    show_bars: Option<Callback<()>>,
    show_logs: Option<Callback<()>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Bars,
    Logs,
}

pub enum RootMsg {
    SwitchMode(Mode),
}

impl Component for Root {
    type Message = RootMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show_bars = link.callback(|()| RootMsg::SwitchMode(Mode::Bars));
        let show_logs = link.callback(|()| RootMsg::SwitchMode(Mode::Logs));
        let repo = YewRepo::new();
        let storage_state = Rc::new(StorageState::load(&repo));

        Self {
            mode: Mode::Bars,
            repo: Rc::new(repo),
            storage_state,
            show_bars: Some(show_bars),
            show_logs: Some(show_logs),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMsg::SwitchMode(new_mode) => {
                let old = self.mode;
                self.mode = new_mode;
                self.mode != old
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
                <Bars storage_state={self.storage_state.clone()} repo={self.repo.clone()} show_logs={self.show_logs.as_ref().expect("logs_cb")} />
            },
            Mode::Logs => html! {
                <Logs storage_state={self.storage_state.clone()} repo={self.repo.clone()} show_bars={self.show_bars.as_ref().expect("bars_cb")} />
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
