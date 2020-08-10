use crate::*;

pub mod bars;
pub mod logs;

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
                <Bars show_logs={self.show_logs.as_ref().expect("logcb")}/>
            },
            Mode::Logs => todo!(),
        }
    }
}

pub struct State {
    mood_readings: Vec<MoodReading>,
    sleep_entries: Vec<TextSubmission>,
    notes: Vec<TextSubmission>,
}
