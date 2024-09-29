use std::collections::HashMap;

use crate::*;
use super::storage_state::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use web::entry::{derive_entries, Entry};
use yew::Component;

pub struct QuickMeds {
    pub choice: Option<QuickMedChoice>,
    pub current_time: Option<NaiveDateTime>,
    mode: QuickMedsMode,
    med_entries: HashMap<NaiveDate, Vec<Entry>>
}

pub enum QuickMedMsg {
    ShowHome,
    ToggleConfig
}

#[derive(PartialEq)]
pub enum QuickMedsMode {
    Entry,
    Config
}

pub struct QuickMedChoice {}

#[derive(Properties, Clone, PartialEq)]
pub struct QuickMedProps {
    pub show_home: Callback<()>,
    pub storage_state: StorageState
}

impl Component for QuickMeds {
    type Message = QuickMedMsg;
    type Properties = QuickMedProps;

    fn create(ctx: &Context<Self>) -> Self {
        // TODO filter
        // TODO filter
        // TODO filter
        let med_entries = derive_entries(&ctx.props().storage_state);
        // TODO filter
        // TODO filter
        // TODO filter

        Self {
            choice: None,
            current_time: None,
            mode: QuickMedsMode::Entry,
            med_entries
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            QuickMedMsg::ShowHome =>  {
                ctx.props().show_home.emit(());
                false
            }
            QuickMedMsg::ToggleConfig => {
                self.mode = match self.mode {
                    QuickMedsMode::Entry => QuickMedsMode::Config,
                    _ => QuickMedsMode::Entry,
                };
                true
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
            <div class="quick-meds-nav">
                <button class="fancy-button thick center" role="button" onclick={ctx.link().callback(|_| QuickMedMsg::ToggleConfig)}>{ if self.mode == QuickMedsMode::Entry { "Config 🛠️"} else { "Entries 💊"} }</button>
            </div>
            </div>
            

            { if self.mode == QuickMedsMode::Config { html! { <></> } } 
              else {
                html! {
                    <>
                    <div id="quick-meds-container">
                    <div id="quick-meds-left">
                    <div id="quick-meds-grid-outer">
                        <div id="quick-meds-grid">
                            <div class="quick-meds-button center">
                                <div class="quick-meds-button-inner">
                                    <button class="fancy-button thick center" role="button">{ "Lamotragine 200mg"}</button>
                                </div>
                            </div>
                            <div class="quick-meds-button center">
                                <div class="quick-meds-button-inner">
                                    <button class="fancy-button thick center" role="button">{ "Lamotragine 200mg"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                    </div>
                    <div id="quick-meds-right">
                        <div id="quick-meds-grid">
                            <div class="quick-meds-log">
                                { "💊 Lamotargine 200mg" }
                            </div>
                            <div class="quick-meds-log">
                                { "💊 Latuda 20mg" }
                            </div>
                            <div class="quick-meds-log">
                                { "💊 Lamotragine 200mg" }
                            </div>
                            <div class="quick-meds-log">
                                { "💊 Latuda 20mg" }
                            </div>
                        </div>
                    </div>
                    </div>
                </>
                }
              } 
        }
        </>
    }
    }
}