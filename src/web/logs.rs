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
        todo!()
    }
    fn view(&self) -> Html {
        todo!()
    }
}
