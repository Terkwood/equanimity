use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::Component;

pub struct LogQuickMed {
    pub choice: Option<QuickMedChoice>,
    pub current_time: Option<NaiveDateTime>,
}

pub struct QuickMedMsg {}

pub struct QuickMedChoice {}

#[derive(Properties, Clone, PartialEq)]
pub struct QuickMedProps {}

impl Component for LogQuickMed {
    type Message = QuickMedMsg;
    type Properties = QuickMedProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            choice: None,
            current_time: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div id="quick-meds">
                <div id="quick-meds-selection">
                </div>
                <div id="quick-meds-log">
                </div>
            </div>
            </>
        }
    }
}