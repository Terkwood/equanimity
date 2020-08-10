use crate::*;
use repo::YewRepo;

pub mod bars;
pub mod logs;

pub struct RootModel {
    mode: Mode,
    show_bars_cb: Option<Callback<()>>,
    show_logs_cb: Option<Callback<()>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Bars,
    Logs,
}

pub struct SwitchModeMsg(Mode);

impl Component for RootModel {
    type Message = SwitchModeMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show_bars_cb = link.callback(|()| SwitchModeMsg(Mode::Bars));
        let show_logs_cb = link.callback(|()| SwitchModeMsg(Mode::Logs));
        Self {
            mode: Mode::Bars,
            show_bars_cb: Some(show_bars_cb),
            show_logs_cb: Some(show_logs_cb),
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
                <bars::BarsModel show_logs_cb={self.show_logs_cb.as_ref().expect("logcb")}/>
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
