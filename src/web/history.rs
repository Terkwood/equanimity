use super::StorageState;
use crate::*;

pub struct History {
    link: ComponentLink<Self>,
    text_area: String,
    top_view: HistoryTopView,
    show_history: bool,
    props: HistoryProps,
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
    ShowHistory,
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
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            top_view: HistoryTopView::MoodButtons,
            text_area: "".to_string(),
            show_history: true,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HistoryMsg::AddReading(r) => {
                self.top_view = HistoryTopView::MoodButtons;
                self.show_history = true;
                self.props.add_mood_reading.emit(r);
                true
            }
            HistoryMsg::TextAreaUpdated(s) => {
                self.text_area = s;
                true
            }
            HistoryMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    self.props
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
                    self.props
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
                    self.props
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
                self.props.show_logs.emit(());
                false
            }
            HistoryMsg::ShowHistory => {
                self.show_history = true;
                true
            }
            HistoryMsg::FocusInput => {
                self.show_history = false;
                self.top_view = HistoryTopView::FocusedOnText;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_top_view() }
                { if self.show_history { html! {
                    <>
                    <br/>
                    {
                        pips::group_by_day(&self.props.storage_state.mood_readings).iter().map(|(day, readings)| {
                            html! {
                                <>
                                    <div class="day-container">
                                        <div class="day-content">
                                            { pips::day_label(day) }
                                            { pips::circles(&readings) }
                                            { pips::day_label(day) }
                                        </div>
                                    </div>
                                </>
                            }
                         }).collect::<Html>()
                    }

                    </>

                }} else {
                    html!{ <></> }
                }}
            </div>
        }
    }
}

impl History {
    fn render_top_view(&self) -> Html {
        match self.top_view {
            HistoryTopView::MoodButtons => html! {
                <>
                    <div id="moodbuttongrid">
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(-3)))>{ "🏥 3️⃣ 🏥" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(-2)))>{ "😭 2️⃣ 😭" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(-1)))>{ "😢 1️⃣ 😢" }</button>
                        </div>
                        <div class="center">
                            <button id="zenbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(0)))>{ "☯" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(1)))>{ "⚡ 1️⃣ ⚡" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(2)))>{ "🔥 2️⃣ 🔥" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| HistoryMsg::AddReading(MoodReading::new(3)))>{ "🤯 3️⃣ 🤯" }</button>
                        </div>
                    </div>
                    <div id="belowmoodbuttongrid">
                        <div class="center">
                            <button class="thick" onclick=self.link.callback(|_| HistoryMsg::ToggleTopView)>{ "Write 🖊"}</button>
                        </div>
                        <div class="center">
                            <button class="thick" onclick=self.link.callback(|_| HistoryMsg::ShowLogs)>{ "View Log 📚"}</button>
                        </div>
                    </div>
                </>
            },
            HistoryTopView::WaitingForText | HistoryTopView::FocusedOnText => {
                let button_class = text_entry_button_class(&self.top_view);
                html! {
                    <div id=format!("controlgrid{}", match self.top_view { HistoryTopView::FocusedOnText => "full", _ => "mini" })>
                        <div id="bigtextgrid">
                            <textarea
                                rows=6
                                value=&self.text_area
                                onfocus=self.link.callback(|_| HistoryMsg::FocusInput)
                                onchange=self.link.callback(|_| HistoryMsg::ShowHistory)
                                oninput=self.link.callback(|e: InputData| HistoryMsg::TextAreaUpdated(e.value))
                                placeholder="Greetings.">
                            </textarea>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| HistoryMsg::ToggleTopView)>{ "History 📊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| HistoryMsg::SubmitSleep)>{ "Sleep 😴" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| HistoryMsg::SubmitMeds)>{ "Meds 💊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| HistoryMsg::SubmitNotes)>{ "Notes 🖊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| HistoryMsg::ShowLogs)>{ "Logs 📚"}</button>
                        </div>
                    </div>
                }
            }
        }
    }
}

const TEXT_ENTRY_BUTTON_FOCUSED: &str = "lookgoodfocused";
const TEXT_ENTRY_BUTTON_DEFAULT: &str = "expandheight";
fn text_entry_button_class(top_view: &HistoryTopView) -> &'static str {
    match top_view {
        HistoryTopView::FocusedOnText => TEXT_ENTRY_BUTTON_FOCUSED,
        _ => TEXT_ENTRY_BUTTON_DEFAULT,
    }
}
