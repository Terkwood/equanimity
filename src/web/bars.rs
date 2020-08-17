use super::{web::utc_now, StorageState};
use crate::*;
use repo::YewRepo;
use std::rc::Rc;
use web::time::js_local_datetime;

pub struct Bars {
    link: ComponentLink<Self>,
    storage_state: StorageState,
    text_area: String,
    top_view: BarsTopView,
    show_logs: Callback<()>,
    submit_mood_reading: Callback<MoodReading>,
    submit_text: Callback<(TextType, String)>,
}

pub enum BarsTopView {
    MoodButtons,
    Writing,
}

pub enum BarsMsg {
    AddReading(MoodReading),
    TextAreaUpdated(String),
    SubmitSleep,
    SubmitMeds,
    SubmitNotes,
    ShowLogs,
    ToggleTopView,
}

#[derive(Properties, Clone)]
pub struct BarsProps {
    pub show_logs: Callback<()>,
    pub submit_mood_reading: Callback<MoodReading>,
    pub submit_text: Callback<(TextType, String)>,
    pub storage_state: StorageState,
}

impl Component for Bars {
    type Message = BarsMsg;
    type Properties = BarsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            top_view: BarsTopView::MoodButtons,
            storage_state: props.storage_state,
            text_area: "".to_string(),
            show_logs: props.show_logs,
            submit_mood_reading: props.submit_mood_reading,
            submit_text: props.submit_text,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BarsMsg::AddReading(r) => {
                self.submit_mood_reading.emit(r);
                // TODO can it be false ?
                true
            }
            BarsMsg::TextAreaUpdated(s) => {
                self.text_area = s;
                true
            }
            BarsMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    self.submit_text
                        .emit((TextType::Sleep, self.text_area.clone()));
                    self.text_area = "".to_string();
                    // TODO update?
                    true
                } else {
                    false
                }
            }
            BarsMsg::SubmitMeds => {
                if !self.text_area.is_empty() {
                    self.text_area = "".to_string();
                    self.submit_text
                        .emit((TextType::Meds, self.text_area.clone()));

                    // todo update?
                    true
                } else {
                    false
                }
            }
            BarsMsg::SubmitNotes => {
                if !self.text_area.is_empty() {
                    self.text_area = "".to_string();
                    // todo update?
                    true
                } else {
                    false
                }
            }
            BarsMsg::ToggleTopView => {
                self.top_view = match self.top_view {
                    BarsTopView::MoodButtons => BarsTopView::Writing,
                    _ => BarsTopView::MoodButtons,
                };
                true
            }
            BarsMsg::ShowLogs => {
                self.show_logs.emit(());
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
        let rms = moods::recent(
            &self.storage_state.mood_readings,
            utc_now(),
            js_local_datetime,
        );
        html! {
            <div>
                { self.render_top_view() }

                <div id="moodgrid">
                    { rms.iter().map(render_mood_bar).collect::<Html>() }
                </div>

                <div id="dategrid">
                    { rms.iter().map(render_mood_date).collect::<Html>() }
                </div>
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
            BarsTopView::Writing => html! {
                <div id="controlgrid">
                    <div id="bigtextgrid">
                        <textarea
                            rows=6
                            value=&self.text_area
                            oninput=self.link.callback(|e: InputData| BarsMsg::TextAreaUpdated(e.value))
                            placeholder="Greetings.">
                        </textarea>
                    </div>
                    <div class="center">
                        <button class="expandheight" onclick=self.link.callback(|_| BarsMsg::ToggleTopView)>{ "Bars 📊" }</button>
                    </div>
                    <div class="center">
                        <button class="expandheight" onclick=self.link.callback(|_| BarsMsg::SubmitSleep)>{ "Sleep 😴" }</button>
                    </div>
                    <div class="center">
                        <button class="expandheight" onclick=self.link.callback(|_| BarsMsg::SubmitMeds)>{ "Meds 💊" }</button>
                    </div>
                    <div class="center">
                        <button class="expandheight" onclick=self.link.callback(|_| BarsMsg::SubmitNotes)>{ "Notes 🖊" }</button>
                    </div>
                    <div class="center">
                        <button class="expandheight" onclick=self.link.callback(|_| BarsMsg::ShowLogs)>{ "Logs 📚"}</button>
                    </div>
                </div>
            },
        }
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
