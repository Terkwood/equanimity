use crate::*;
use repo::YewRepo;

pub mod bars;
pub mod logs;

pub struct RootModel;

pub enum Mode {
    Bars,
    Logs,
}

pub struct SwitchMode(Mode);

impl Component for RootModel {
    type Message = SwitchMode;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
    fn view(&self) -> Html {
        todo!()
    }
}

pub struct State {
    mood_readings: Vec<MoodReading>,
    sleep_entries: Vec<TextSubmission>,
    notes: Vec<TextSubmission>,
}
