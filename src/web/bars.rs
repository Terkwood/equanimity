use super::{web::utc_now, StorageState};
use crate::*;
use repo::YewRepo;
use web::time::local_datetime;

pub struct Bars {
    link: ComponentLink<Self>,
    repo: YewRepo,
    state: StorageState,
    text_area: String,
    top_view: BarsTopView,
    show_logs: Callback<()>,
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
}

impl Component for Bars {
    type Message = BarsMsg;
    type Properties = BarsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let repo = YewRepo::new();
        let state = StorageState::load(&repo);

        Self {
            link,
            repo,
            top_view: BarsTopView::MoodButtons,
            state,
            text_area: "".to_string(),
            show_logs: props.show_logs,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BarsMsg::AddReading(r) => {
                self.state.mood_readings.push(r);
                self.repo
                    .save_mood_readings(&self.state.mood_readings)
                    .expect("save mood readings");
                true
            }
            BarsMsg::TextAreaUpdated(s) => {
                self.text_area = s;
                true
            }
            BarsMsg::SubmitSleep => {
                if !self.text_area.is_empty() {
                    self.state
                        .sleep_entries
                        .push(TextSubmission::new(self.text_area.clone()));
                    self.text_area = "".to_string();
                    self.repo
                        .save_text(TextType::Sleep, &self.state.sleep_entries)
                        .expect("save sleep")
                }
                true
            }
            BarsMsg::SubmitMeds => {
                if !self.text_area.is_empty() {
                    self.state
                        .meds
                        .push(TextSubmission::new(self.text_area.clone()));
                    self.text_area = "".to_string();
                    self.repo
                        .save_text(TextType::Meds, &self.state.meds)
                        .expect("save meds")
                }
                true
            }
            BarsMsg::SubmitNotes => {
                if !self.text_area.is_empty() {
                    self.state
                        .notes
                        .push(TextSubmission::new(self.text_area.clone()));
                    self.text_area = "".to_string();
                    self.repo
                        .save_text(TextType::Notes, &self.state.notes)
                        .expect("save notes")
                }
                true
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
        let rms = moods::recent(utc_now(), &self.state.mood_readings);
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
    let dt = local_datetime(r.epoch_millis);
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
