use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::Component;

pub struct QuickMeds {
    pub choice: Option<QuickMedChoice>,
    pub current_time: Option<NaiveDateTime>,
}

pub enum QuickMedMsg {
    ShowHome
}

pub struct QuickMedChoice {}

#[derive(Properties, Clone, PartialEq)]
pub struct QuickMedProps {
    pub show_home: Callback<()>
}

impl Component for QuickMeds {
    type Message = QuickMedMsg;
    type Properties = QuickMedProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            choice: None,
            current_time: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            QuickMedMsg::ShowHome =>  {
                ctx.props().show_home.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="center">
            <div class="quick-meds-nav">
                <button class="fancy-button thick center" role="button" onclick={ctx.link().callback(|_| QuickMedMsg::ShowHome)}>{ "Home 🔵🔴"}</button>
            </div>
            </div>
            <div id="quick-meds-container">
            <div id="quick-meds-left">
            <div id="quick-meds-grid-outer">
                <div id="quick-meds-grid-inner">
                    <div class="log-entry">
                    <button class="fancy-button thick center" role="button"/>
                    </div>
                </div>
            </div>
            </div>
            <div id="quick-meds-right">
                <div id="quick-meds-grid-inner">
                    <div class="log-entry">
                        { "💊 Lamotrgn 200mg" }
                    </div>
                    <div class="log-entry">
                        { "💊 Latuda 20mg" }
                    </div>
                    <div class="log-entry">
                        { "💊 Lamotrgn 200mg" }
                    </div>
                    <div class="log-entry">
                        { "💊 Latuda 20mg" }
                    </div>
                </div>
            </div>
            </div>
            </>
        }
    }
}