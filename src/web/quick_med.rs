use super::storage_state::*;
use crate::*;
use chrono::Utc;
use web::entry::{derive_entries, Entry};
use web_sys::HtmlTextAreaElement;
use yew::html::onchange;
use yew::Component;

pub struct QuickMeds {
    mode: QuickMedsMode,
    entries: Vec<TextSubmission>,
    buttons: Vec<QuickMedButton>,
    text_area: String,
}

pub enum QuickMedMsg {
    ShowHome,
    ToggleConfig,
    DeleteButton(QuickMedButton),
    AddButton,
    ClickButton(QuickMedButton),
    TextAreaUpdated(String),
}

#[derive(PartialEq)]
pub enum QuickMedsMode {
    Entry,
    Config,
}

#[derive(Properties, Clone, PartialEq)]
pub struct QuickMedProps {
    pub show_home: Callback<()>,
    pub add_button: Callback<QuickMedButton>,
    pub delete_button: Callback<QuickMedButton>,
    pub storage_state: StorageState,
    pub log_med: Callback<(TextType, String)>,
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
            mode: QuickMedsMode::Entry,
            entries: med_entries,
            buttons: med_buttons.to_vec(),
            text_area: "".to_string(),
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
            QuickMedMsg::DeleteButton(button) => {
                self.delete_button(&button);
                ctx.props().delete_button.emit(button.clone());
                true
            }
            QuickMedMsg::AddButton => {
                if !self.text_area.is_empty() {
                    self.add_button(QuickMedButton(self.text_area.clone()));
                    self.text_area = "".to_string();
                }

                true
            }
            QuickMedMsg::TextAreaUpdated(s) => {
                self.text_area.push_str(&s);
                true
            }
            QuickMedMsg::ClickButton(button) => {
                ctx.props()
                        .log_med
                        .emit((TextType::Meds, button.0.clone()));
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


                { if self.mode == QuickMedsMode::Config {
                    html!
                    { <>
                        <div id="controlgridmini">
                        <div id="bigtextgrid">
                            <textarea
                                rows=6
                                value={self.text_area.clone()}
                                onchange={on_change_callback(ctx)}
                                placeholder="Add a button.">
                            </textarea>
                        </div>
                        <div class="center">
                            <button class="fancy-button thick" onclick={ctx.link().callback(|_| QuickMedMsg::AddButton)}>{ "Add Button 🔤" }</button>
                        </div>
                    </div>
                      { self.buttons.iter().map(|b|self.render_button_config(&ctx, b.clone())).collect::<Html>() }
                      </>
                    }
                  } else {
                    html! {
                        <>
                        <div id="quick-meds-container">
                        <div id="quick-meds-left">
                        <div id="quick-meds-grid-outer">
                            <div id="quick-meds-grid">
                                {self.buttons.iter().map(|button| self.render_button(ctx, button)).collect::<Html>() }
                            </div>
                        </div>
                        </div>
                        <div id="quick-meds-right">
                            <div id="quick-meds-grid">
                                { self.render_day_meds(self.entries.clone())}
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
    fn add_button(&mut self, button: QuickMedButton) {
        self.buttons.push(button);
    }

    fn delete_button(&mut self, button: &QuickMedButton) {
        self.buttons = self
            .buttons
            .clone()
            .into_iter()
            .filter(|b| b != button)
            .collect::<Vec<QuickMedButton>>();
    }

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
        <button class="fancy-button" role="button" onclick={ctx.link().callback(move |_| QuickMedMsg::DeleteButton(b.clone()))}>{ "🗑️" }</button>
        </>}
    }

    fn render_button(&self, ctx: &yew::Context<Self>, button: &QuickMedButton) -> Html {
        let bc = button.clone();
        html! {
         <>
         <div class="quick-meds-button center">
             <div class="quick-meds-button-inner">
                 <button class="fancy-button thick center" role="button"
                         onclick={ctx.link().callback(move |_| QuickMedMsg::ClickButton(bc.clone()))}>
                   { button.clone().0 }
                </button>
             </div>
         </div>
         </>
        }
    }
}

fn on_change_callback(ctx: &yew::Context<QuickMeds>) -> Callback<Event> {
    ctx.link().callback(|e: onchange::Event| {
        QuickMedMsg::TextAreaUpdated(
            e.target()
                .map(|t| t.value_of())
                .map(|o| o.dyn_into::<HtmlTextAreaElement>())
                .map(|text_area_elem| text_area_elem.unwrap().value())
                .unwrap_or_default(),
        )
    })
}
