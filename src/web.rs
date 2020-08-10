use crate::*;
use repo::YewRepo;

pub struct State {
    mood_readings: Vec<MoodReading>,
    sleep_entries: Vec<TextSubmission>,
    notes: Vec<TextSubmission>,
    text_area: String,
}

pub struct Model {
    link: ComponentLink<Self>,
    repo: YewRepo,
    state: State,
}

pub enum Msg {
    AddReading(MoodReading),
    TextAreaUpdated(String),
    SubmitSleep,
    SubmitNotes,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let repo = YewRepo::new();

        let state = State {
            mood_readings: repo.load_mood_readings().unwrap_or(vec![]),
            sleep_entries: repo.load_sleep().unwrap_or(vec![]),
            notes: repo.load_notes().unwrap_or(vec![]),
            text_area: "".to_string(),
        };

        Self { link, repo, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddReading(r) => {
                self.state.mood_readings.push(r);
                self.repo
                    .save_mood_readings(&self.state.mood_readings)
                    .expect("save mood readings")
            }
            Msg::TextAreaUpdated(s) => self.state.text_area = s,
            Msg::SubmitSleep => {
                if !self.state.text_area.is_empty() {
                    self.state
                        .sleep_entries
                        .push(TextSubmission::new(self.state.text_area.clone()));
                    self.state.text_area = "".to_string();
                    self.repo
                        .save_sleep(&self.state.sleep_entries)
                        .expect("save sleep")
                }
            }
            Msg::SubmitNotes => {
                if !self.state.text_area.is_empty() {
                    self.state
                        .notes
                        .push(TextSubmission::new(self.state.text_area.clone()));
                    self.state.text_area = "".to_string();
                    self.repo.save_notes(&self.state.notes).expect("save notes")
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let rms = moods::recent(now(), &self.state.mood_readings);
        html! {
            <div>
                <div id="controlgrid">
                    <div>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(3)))>{ "🤯 3 🤯" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(2)))>{ "🔥 2 🔥" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(1)))>{ "⚡ 1 ⚡" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(0)))>{ "☯ 🧘 ☯" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-1)))>{ "😢 1 😢" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-2)))>{ "😭 2 😭" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-3)))>{ "🏥 3 🏥" }</button>


                    </div>

                    <div id="bigtext">
                        <textarea
                            rows=6
                            value=&self.state.text_area
                            oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                            placeholder="Greetings.">
                        </textarea>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitSleep)>{ "Submit 😴" }</button>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitNotes)>{ "Submit 🖊" }</button>
                        <p> { "Sleep: " } { &self.state.sleep_entries.len() } { " Notes: " } { &self.state.notes.len() }</p>
                    </div>
                </div>

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
    let dt = Utc.timestamp_millis(r.epoch_millis as i64);
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
