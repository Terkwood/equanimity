use super::State;
use crate::*;
use repo::YewRepo;

pub struct LogsModel {
    link: ComponentLink<Self>,
    repo: YewRepo,
    state: State,
}

pub enum LogsMsg {}

impl Component for LogsModel {
    type Message = LogsMsg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        todo!()
    }
}
