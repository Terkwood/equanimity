use super::State;
use crate::*;
use repo::YewRepo;
use std::rc::Rc;

pub struct Logs {
    link: ComponentLink<Self>,
    repo: YewRepo,
    state: State,
    show_bars: Callback<()>,
}

pub enum LogsMsg {
    ShowBars,
}

#[derive(Properties, Clone)]
pub struct LogsProps {
    pub show_bars: Callback<()>,
    pub repo: Rc<YewRepo>,
}

impl Component for Logs {
    type Message = LogsMsg;
    type Properties = LogsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let repo = YewRepo::new();

        let state = State {
            mood_readings: props.repo.load_mood_readings().unwrap_or(vec![]),
            sleep_entries: props.repo.load_sleep().unwrap_or(vec![]),
            notes: props.repo.load_notes().unwrap_or(vec![]),
        };

        Self {
            link,
            state,
            repo,
            show_bars: props.show_bars,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LogsMsg::ShowBars => {
                self.show_bars.emit(());
                false
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
        html! {
            <button onclick=self.link.callback(|_| LogsMsg::ShowBars)>{ "Show Bars 📊"}</button>
        }
    }
}
