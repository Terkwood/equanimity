use super::StorageState;
use crate::*;
use web_sys::HtmlTextAreaElement;
use yew::html::onchange;

pub struct History {
    text_area: String,
    top_view: HistoryTopView,
    show_history: bool,
}

pub enum HistoryTopView {
    MoodButtons,
    WaitingForText,
    FocusedOnText,
}

pub enum HistoryMsg {
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
pub struct HistoryProps {
    pub show_logs: Callback<()>,
    pub add_mood_reading: Callback<MoodReading>,
    pub add_text: Callback<(TextType, String)>,
    pub storage_state: StorageState,
}

impl Component for History {
    type Message = HistoryMsg;
    type Properties = HistoryProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            top_view: HistoryTopView::MoodButtons,
            text_area: "".to_string(),
            show_history: true,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HistoryMsg::AddReading(r) => {
                self.top_view = HistoryTopView::MoodButtons;
                self.show_history = true;
                ctx.props().add_mood_reading.emit(r);
                true
            }
            HistoryMsg::TextAreaUpdated(s) => {
                self.text_area.push_str(&s);
                true
            }
            HistoryMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Sleep, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = HistoryTopView::MoodButtons;
                self.show_history = true;

                true
            }
            HistoryMsg::SubmitMeds => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Meds, self.text_area.clone()));
                    self.text_area = "".to_string();
                }

                self.top_view = HistoryTopView::MoodButtons;
                self.show_history = true;

                true
            }
            HistoryMsg::SubmitNotes => {
                if !self.text_area.is_empty() {
                    ctx.props()
                        .add_text
                        .emit((TextType::Notes, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = HistoryTopView::MoodButtons;
                self.show_history = true;

                true
            }
            HistoryMsg::ToggleTopView => {
                self.top_view = match self.top_view {
                    HistoryTopView::MoodButtons => HistoryTopView::WaitingForText,
                    _ => HistoryTopView::MoodButtons,
                };
                true
            }
            HistoryMsg::ShowLogs => {
                ctx.props().show_logs.emit(());
                false
            }
            HistoryMsg::FocusInput => {
                self.show_history = false;
                self.top_view = HistoryTopView::FocusedOnText;
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

    fn view(&self, ctx: &yew::Context<History>) -> Html {
        html! {
            <div>
                { self.render_top_view(ctx) }
                { if self.show_history { html! {
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
                                        <div class="piplabel">{ pips::day_label(day) }</div>
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

impl History {
    fn render_top_view(&self, ctx: &yew::Context<Self>) -> Html {
        match self.top_view {
            HistoryTopView::MoodButtons => html! {
                <>
                    <div id="mood-button-grid">
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(-3)))}>{ "🏥 3️⃣ 🏥" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(-2)))}>{ "😭 2️⃣ 😭" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(-1)))}>{ "😢 1️⃣ 😢" }</button>
                        </div>
                        <div class="center">
                            <button id="equanimity-button" class="fancy-button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(0)))}>{ "☯" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(1)))}>{ "⚡ 1️⃣ ⚡" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(2)))}>{ "🔥 2️⃣ 🔥" }</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| HistoryMsg::AddReading(MoodReading::new(3)))}>{ "🤯 3️⃣ 🤯" }</button>
                        </div>
                    </div>
                    <div id="below-mood-button-grid">
                        <div class="center">
                            <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| HistoryMsg::ToggleTopView)}>{ "Write 🖊"}</button>
                        </div>
                        <div class="center">
                            <button class="fancy-button thick" role="button" onclick={ctx.link().callback(|_| HistoryMsg::ShowLogs)}>{ "View Log 📚"}</button>
                        </div>
                    </div>
                </>
            },
            HistoryTopView::WaitingForText | HistoryTopView::FocusedOnText => {
                let button_class = text_entry_button_class(&self.top_view);
                html! {
                    <div id={format!("controlgrid{}", match self.top_view { HistoryTopView::FocusedOnText => "full", _ => "mini" })}>
                        <div id="bigtextgrid">
                            <textarea
                                rows=6
                                value={self.text_area.clone()}
                                onfocus={ctx.link().callback(|_| HistoryMsg::FocusInput)}
                                onchange={on_change_callback(ctx)}
                                placeholder="Greetings.">
                            </textarea>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HistoryMsg::ToggleTopView)}>{ "Exit 🚫" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HistoryMsg::SubmitSleep)}>{ "Sleep 😴" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HistoryMsg::SubmitMeds)}>{ "Meds 💊" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HistoryMsg::SubmitNotes)}>{ "Notes 🖊" }</button>
                        </div>
                        <div class="center">
                            <button class={button_class} onclick={ctx.link().callback(|_| HistoryMsg::ShowLogs)}>{ "Logs 📚"}</button>
                        </div>
                    </div>
                }
            }
        }
    }
}

const TEXT_ENTRY_BUTTON_FOCUSED: &str = "fancy-button";
const TEXT_ENTRY_BUTTON_DEFAULT: &str = "fancy-button";
fn text_entry_button_class(top_view: &HistoryTopView) -> &'static str {
    match top_view {
        HistoryTopView::FocusedOnText => TEXT_ENTRY_BUTTON_FOCUSED,
        _ => TEXT_ENTRY_BUTTON_DEFAULT,
    }
}

fn on_change_callback(ctx: &yew::Context<History>) -> Callback<Event> {
    ctx.link().callback(|e: onchange::Event| {
        HistoryMsg::TextAreaUpdated(
            e.target()
                .map(|t| t.value_of())
                .map(|o| o.dyn_into::<HtmlTextAreaElement>())
                .map(|text_area_elem| text_area_elem.unwrap().value())
                .unwrap_or_default(),
        )
    })
}
