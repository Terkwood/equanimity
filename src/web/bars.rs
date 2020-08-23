use super::{web::utc_now, StorageState};
use crate::*;
use web::time::js_local_datetime;

pub struct Bars {
    link: ComponentLink<Self>,
    text_area: String,
    top_view: BarsTopView,
    show_bars: bool,
    props: BarsProps,
}

pub enum BarsTopView {
    MoodButtons,
    WaitingForText,
    FocusedOnText,
}

pub enum BarsMsg {
    AddReading(MoodReading),
    TextAreaUpdated(String),
    SubmitSleep,
    SubmitMeds,
    SubmitNotes,
    ShowLogs,
    ToggleTopView,
    ShowBars,
    FocusInput,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BarsProps {
    pub show_logs: Callback<()>,
    pub add_mood_reading: Callback<MoodReading>,
    pub add_text: Callback<(TextType, String)>,
    pub storage_state: StorageState,
}

impl Component for Bars {
    type Message = BarsMsg;
    type Properties = BarsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            top_view: BarsTopView::MoodButtons,
            text_area: "".to_string(),
            show_bars: true,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BarsMsg::AddReading(r) => {
                self.top_view = BarsTopView::MoodButtons;
                self.show_bars = true;
                self.props.add_mood_reading.emit(r);
                true
            }
            BarsMsg::TextAreaUpdated(s) => {
                self.text_area = s;
                true
            }
            BarsMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    self.props
                        .add_text
                        .emit((TextType::Sleep, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = BarsTopView::MoodButtons;
                self.show_bars = true;

                true
            }
            BarsMsg::SubmitMeds => {
                if !self.text_area.is_empty() {
                    self.props
                        .add_text
                        .emit((TextType::Meds, self.text_area.clone()));
                    self.text_area = "".to_string();
                }

                self.top_view = BarsTopView::MoodButtons;
                self.show_bars = true;

                true
            }
            BarsMsg::SubmitNotes => {
                if !self.text_area.is_empty() {
                    self.props
                        .add_text
                        .emit((TextType::Notes, self.text_area.clone()));
                    self.text_area = "".to_string();
                }
                self.top_view = BarsTopView::MoodButtons;
                self.show_bars = true;

                true
            }
            BarsMsg::ToggleTopView => {
                self.top_view = match self.top_view {
                    BarsTopView::MoodButtons => BarsTopView::WaitingForText,
                    _ => BarsTopView::MoodButtons,
                };
                true
            }
            BarsMsg::ShowLogs => {
                self.props.show_logs.emit(());
                false
            }
            BarsMsg::ShowBars => {
                self.show_bars = true;
                true
            }
            BarsMsg::FocusInput => {
                self.show_bars = false;
                self.top_view = BarsTopView::FocusedOnText;
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
        let rms = moods::recent(
            &self.props.storage_state.mood_readings,
            utc_now(),
            js_local_datetime,
        );
        html! {
            <div>
                { self.render_top_view() }
                { if self.show_bars { html! {
                    <>
                    <div id="moodgrid">
                        { rms.iter().map(render_mood_bar).collect::<Html>() }
                    </div>

                    <div id="dategrid">
                        { rms.iter().map(render_mood_date).collect::<Html>() }
                    </div>
                    </>
                }} else {
                    html!{ <></> }
                }}
            </div>
        }
    }
}

impl Bars {
    fn render_top_view(&self) -> Html {
        match self.top_view {
            BarsTopView::MoodButtons => html! {
                <>
                    <div id="moodbuttongrid">
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(-3)))>{ "🏥 3️⃣ 🏥" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(-2)))>{ "😭 2️⃣ 😭" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(-1)))>{ "😢 1️⃣ 😢" }</button>
                        </div>
                        <div class="center">
                            <button id="zenbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(0)))>{ "☯" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(1)))>{ "⚡ 1️⃣ ⚡" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(2)))>{ "🔥 2️⃣ 🔥" }</button>
                        </div>
                        <div class="center">
                            <button class="moodbutton" onclick=self.link.callback(|_| BarsMsg::AddReading(MoodReading::new(3)))>{ "🤯 3️⃣ 🤯" }</button>
                        </div>
                    </div>
                    <div id="belowmoodbuttongrid">
                        <div class="center">
                            <button class="thick" onclick=self.link.callback(|_| BarsMsg::ToggleTopView)>{ "Write 🖊"}</button>
                        </div>
                        <div class="center">
                            <button class="thick" onclick=self.link.callback(|_| BarsMsg::ShowLogs)>{ "View Log 📚"}</button>
                        </div>
                    </div>
                </>
            },
            BarsTopView::WaitingForText | BarsTopView::FocusedOnText => {
                let button_class = text_entry_button_class(&self.top_view);
                html! {
                    <div id=format!("controlgrid{}", match self.top_view { BarsTopView::FocusedOnText => "full", _ => "mini" })>
                        <div id="bigtextgrid">
                            <textarea
                                rows=6
                                value=&self.text_area
                                onfocus=self.link.callback(|_| BarsMsg::FocusInput)
                                onchange=self.link.callback(|_| BarsMsg::ShowBars)
                                oninput=self.link.callback(|e: InputData| BarsMsg::TextAreaUpdated(e.value))
                                placeholder="Greetings.">
                            </textarea>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| BarsMsg::ToggleTopView)>{ "Bars 📊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| BarsMsg::SubmitSleep)>{ "Sleep 😴" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| BarsMsg::SubmitMeds)>{ "Meds 💊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| BarsMsg::SubmitNotes)>{ "Notes 🖊" }</button>
                        </div>
                        <div class="center">
                            <button class=button_class onclick=self.link.callback(|_| BarsMsg::ShowLogs)>{ "Logs 📚"}</button>
                        </div>
                    </div>
                }
            }
        }
    }
}

const TEXT_ENTRY_BUTTON_FOCUSED: &str = "lookgoodfocused";
const TEXT_ENTRY_BUTTON_DEFAULT: &str = "expandheight";
fn text_entry_button_class(top_view: &BarsTopView) -> &'static str {
    match top_view {
        BarsTopView::FocusedOnText => TEXT_ENTRY_BUTTON_FOCUSED,
        _ => TEXT_ENTRY_BUTTON_DEFAULT,
    }
}

fn render_mood_bar(r: &MoodReading) -> Html {
    let value = r.value;
    html! {
        <>
            <div class={class_from(value, 3)}></div>
            <div class={class_from(value, 2)}></div>
            <div class={class_from(value, 1)}></div>
            <div class={class_from(value, 0)}></div>
            <div class={class_from(value, -1)}></div>
            <div class={class_from(value, -2)}></div>
            <div class={class_from(value, -3)}></div>
        </>
    }
}

fn render_mood_date(r: &MoodReading) -> Html {
    let dt = js_local_datetime(r.epoch_millis);
    let date_string = dt.format("%m/%d").to_string();
    html! {
        <>
            <div class="date">{ date_string }</div>
        </>
    }
}

fn class_from(value: i8, position: i8) -> String {
    if position == 0 {
        "neutral".to_string()
    } else if position > 0 && value >= position {
        format!("hot{}", position)
    } else if position < 0 && value <= position {
        format!("cold{}", position.abs())
    } else {
        "nocolor".to_string()
    }
}
