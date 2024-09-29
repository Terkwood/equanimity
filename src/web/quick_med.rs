use std::collections::HashMap;

use super::storage_state::*;
use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use web::entry::{derive_entries, Entry};
use yew::Component;

pub struct QuickMeds {
    pub choice: Option<QuickMedChoice>,
    pub current_time: Option<NaiveDateTime>,
    mode: QuickMedsMode,
    med_entries: Vec<TextSubmission>,
    med_buttons: Vec<QuickMedButton>,
}

pub enum QuickMedMsg {
    ShowHome,
    ToggleConfig,
    Delete(QuickMedButton)
}

#[derive(PartialEq)]
pub enum QuickMedsMode {
    Entry,
    Config,
}

pub struct QuickMedChoice {}

#[derive(Properties, Clone, PartialEq)]
pub struct QuickMedProps {
    pub show_home: Callback<()>,
    pub storage_state: StorageState,
}

impl Component for QuickMeds {
    type Message = QuickMedMsg;
    type Properties = QuickMedProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut med_entries: Vec<TextSubmission> = derive_entries(&ctx.props().storage_state)
            .into_iter()
            .filter(|(k, _)| k == &Utc::now().naive_utc().date())
            .map(|(_, v)| v)
            .flatten()
            .filter(|e| match e {
                Entry::Meds(_) => true,
                _ => false,
            })
            .map(|e| match e {
                Entry::Meds(v) => v,
                _ => unreachable!(),
            })
            .collect();
        med_entries.reverse();

        let med_buttons = &ctx.props().storage_state.quick_med_buttons;

        Self {
            choice: None,
            current_time: None,
            mode: QuickMedsMode::Entry,
            med_entries,
            med_buttons: med_buttons.to_vec(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            QuickMedMsg::ShowHome => {
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
            QuickMedMsg::Delete(button) => {
                todo!()
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


                { if self.mode == QuickMedsMode::Config {
                    html!
                    { <>
                      { self.med_buttons.iter().map(|b|self.render_button_config(&ctx, b.clone())).collect::<Html>() }
                      </>
                    }
                  } else {
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
                                { self.render_day_meds(self.med_entries.clone())}
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

impl QuickMeds {
    fn render_day_meds(&self, day_entries: Vec<TextSubmission>) -> Html {
        html! {
            <>
                { day_entries.iter().map(|t| self.render_med_text( t.clone())).collect::<Html>() }
            </>
        }
    }

    fn render_med_text(&self, t: TextSubmission) -> Html {
        html! {<>
        <div class="quick-meds-log">
                { format!("💊 {}", t.value) }
            </div>
        </>}
    }

    fn render_button_config(&self, ctx: &yew::Context<Self>, b: QuickMedButton) -> Html {
        html! { <>
            <button class="fancy-button" role="button" onclick={ctx.link().callback(move |_| QuickMedMsg::Delete(b.clone()))}>{ "🗑️" }</button>
            </>}
    }
}
