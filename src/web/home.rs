use super::StorageState;
use crate::*;
use web_sys::HtmlTextAreaElement;
use yew::html::onchange;

pub struct Home {
    text_area: String,
    top_view: HomeTopView,
    show_home: bool,
}

pub enum HomeTopView {
    MoodButtons,
    WaitingForText,
    FocusedOnText,
}

pub enum HomeMsg {
    AddReading(MoodReading),
    TextAreaUpdated(String),
    SubmitSleep,
    SubmitMeds,
    SubmitNotes,
    ShowLogs,
    ToggleTopView,
    FocusInput,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub show_logs: Callback<()>,
    pub add_mood_reading: Callback<MoodReading>,
    pub add_text: Callback<(TextType, String)>,
    pub storage_state: StorageState,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = HomeProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            top_view: HomeTopView::MoodButtons,
            text_area: "".to_string(),
            show_home: true,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::AddReading(r) => {
                self.top_view = HomeTopView::MoodButtons;
                self.show_home = true;
                ctx.props().add_mood_reading.emit(r);
                true
            }
            HomeMsg::TextAreaUpdated(s) => {
                self.text_area.push_str(&s);
                true
            }
            HomeMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Sleep, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = HomeTopView::MoodButtons;
                self.show_home = true;

                true
            }
            HomeMsg::SubmitMeds => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Meds, self.text_area.clone()));
                    self.text_area = "".to_string();
                }

                self.top_view = HomeTopView::MoodButtons;
                self.show_home = true;

                true
            }
            HomeMsg::SubmitNotes => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Notes, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = HomeTopView::MoodButtons;
                self.show_home = true;

                true
            }
            HomeMsg::ToggleTopView => {
                self.top_view = match self.top_view {
                    HomeTopView::MoodButtons => HomeTopView::WaitingForText,
                    _ => HomeTopView::MoodButtons,
                };
                self.show_home = !self.show_home;
                true
            }
            HomeMsg::ShowLogs => {
                ctx.props().show_logs.emit(());
                false
            }
            HomeMsg::FocusInput => {
                self.show_home = false;
                self.top_view = HomeTopView::FocusedOnText;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.storage_state != ctx.props().storage_state {
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &yew::Context<Home>) -> Html {
        html! {
            <div>
                { self.render_top_view(ctx) }
                { if self.show_home { html! {
                    <>
                    <br/>
                    <>
                    {
                        pips::group_by_day(&ctx.props().storage_state.mood_readings).iter().map(|(day, readings)| {
                            html! {
                                <>
                                    <div class="day-container">
                                        <div class="piplabel">{ pips::blank_label() }</div>
                                        <div class="pips">{ pips::circles(&readings) }</div>
                                        <div class="piplabel">{NBSP} {NBSP} { trim_year_from_date(day) }</div>
                                    </div>
                                </>
                            }
                         }).collect::<Html>()
                    }
                    </>

                    </>

                }} else {
                    html!{ <></> }
                }}
            </div>
        }
    }
}

fn trim_year_from_date(date: &str) -> String {
    // given a date string like YYYY-MM-DD, return MM-DD
    date.chars().skip(5).collect()
}

impl Home {
    fn render_top_view(&self, ctx: &yew::Context<Self>) -> Html {
        match self.top_view {
            HomeTopView::MoodButtons => html! {
                <>
                    <div id="mood-button-grid">
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(-3)))}>{ "🏥 3️⃣ 🏥" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(-2)))}>{ "😭 2️⃣ 😭" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(-1)))}>{ "😢 1️⃣ 😢" }</button>
                        </div>
                        <div class="center">
                            <button id="equanimity-button" class="fancy-button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(0)))}>{ "☯" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(1)))}>{ "⚡ 1️⃣ ⚡" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(2)))}>{ "🔥 2️⃣ 🔥" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HomeMsg::AddReading(MoodReading::new(3)))}>{ "🤯 3️⃣ 🤯" }</button>
                        </div>
                    </div>
                    <div id="below-mood-button-grid">
                        <div class="center">
                            <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| HomeMsg::ToggleTopView)}>{ "Write 🖊"}</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| HomeMsg::ShowLogs)}>{ "View Log 📚"}</button>
                        </div>
                    </div>
                </>
            },
            HomeTopView::WaitingForText | HomeTopView::FocusedOnText => {
                let button_class = text_entry_button_class(&self.top_view);
                html! {
                    <div id={format!("controlgrid{}", match self.top_view { HomeTopView::FocusedOnText => "full", _ => "mini" })}>
                        <div id="bigtextgrid">
                            <textarea
                                rows=6
                                value={self.text_area.clone()}
                                onfocus={ctx.link().callback(|_| HomeMsg::FocusInput)}
                                onchange={on_change_callback(ctx)}
                                placeholder="Greetings.">
                            </textarea>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HomeMsg::ToggleTopView)}>{ "Exit 🚫" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HomeMsg::SubmitSleep)}>{ "Sleep 😴" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HomeMsg::SubmitMeds)}>{ "Meds 💊" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HomeMsg::SubmitNotes)}>{ "Notes 🖊" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HomeMsg::ShowLogs)}>{ "Logs 📚"}</button>
                        </div>
                    </div>
                }
            }
        }
    }
}

const TEXT_ENTRY_BUTTON_FOCUSED: &str = "fancy-button write-button";
const TEXT_ENTRY_BUTTON_DEFAULT: &str = "fancy-button write-button";
fn text_entry_button_class(top_view: &HomeTopView) -> &'static str {
    match top_view {
        HomeTopView::FocusedOnText => TEXT_ENTRY_BUTTON_FOCUSED,
        _ => TEXT_ENTRY_BUTTON_DEFAULT,
    }
}

fn on_change_callback(ctx: &yew::Context<Home>) -> Callback<Event> {
    ctx.link().callback(|e: onchange::Event| {
        HomeMsg::TextAreaUpdated(
            e.target()
                .map(|t| t.value_of())
                .map(|o| o.dyn_into::<HtmlTextAreaElement>())
                .map(|text_area_elem| text_area_elem.unwrap().value())
                .unwrap_or_default(),
        )
    })
}
