#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    readings: Vec<Reading>,
    sleep_entries: Vec<TextSubmission>,
    sleep_text_area: String,
    notes: Vec<TextSubmission>,
    notes_text_area: String,
}

#[derive(Clone, Debug)]
struct TextSubmission {
    pub value: String,
    pub _epoch_millis: u64,
}

impl TextSubmission {
    pub fn new(value: String) -> Self {
        TextSubmission {
            value,
            _epoch_millis: now(),
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct Reading {
    pub value: i8,
    pub _epoch_millis: u64,
}

fn now() -> u64 {
    js_sys::Date::now() as u64
}
const MIN_READING: i8 = -3;
const MAX_READING: i8 = 3;
impl Reading {
    pub fn new(value: i8) -> Reading {
        let _epoch_millis = now();
        if value < MIN_READING {
            Reading {
                value: MIN_READING,
                _epoch_millis,
            }
        } else if value > MAX_READING {
            Reading {
                value: MAX_READING,
                _epoch_millis,
            }
        } else {
            Reading {
                value,
                _epoch_millis,
            }
        }
    }
    pub fn get(self) -> i8 {
        self.value
    }
}

fn render_bar(value: i8) -> Html {
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

enum Msg {
    AddReading(Reading),
    SleepTextAreaUpdated(String),
    SubmitSleep,
    NotesTextAreaUpdated(String),
    SubmitNotes,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            readings: vec![],
            sleep_entries: vec![],
            sleep_text_area: "".to_string(),
            notes: vec![],
            notes_text_area: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddReading(r) => self.readings.push(r),
            Msg::SleepTextAreaUpdated(s) => self.sleep_text_area = s,
            Msg::SubmitSleep => {
                if !self.sleep_text_area.is_empty() {
                    self.sleep_entries
                        .push(TextSubmission::new(self.sleep_text_area.clone()));
                    self.sleep_text_area = "".to_string()
                }
            }
            Msg::NotesTextAreaUpdated(s) => self.notes_text_area = s,
            Msg::SubmitNotes => {
                if !self.notes_text_area.is_empty() {
                    self.notes
                        .push(TextSubmission::new(self.notes_text_area.clone()));
                    self.notes_text_area = "".to_string()
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
        html! {
            <div>
                <div id="controlgrid">
                    <div>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-3)))>{ "-3" }</button>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-2)))>{ "-2" }</button>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-1)))>{ "-1" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(0)))>{ "0" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(1)))>{ "+1" }</button>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(2)))>{ "+2" }</button>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(Reading::new(3)))>{ "+3" }</button>
                    </div>

                    <div>
                        <textarea rows=2
                            value=&self.sleep_text_area
                            oninput=self.link.callback(|e: InputData| Msg::SleepTextAreaUpdated(e.value))
                            placeholder="how you slept">
                        </textarea>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitSleep)>{ "Submit Sleep" }</button>

                        <p> { "Sleep entries submitted: " } { &self.sleep_entries.len() } </p>
                    </div>

                    <div>
                        <textarea rows=2
                            value=&self.notes_text_area
                            oninput=self.link.callback(|e: InputData| Msg::NotesTextAreaUpdated(e.value))
                            placeholder="notes">
                        </textarea>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitNotes)>{ "Submit Notes" }</button>

                        <p> { "Notes submitted: " } { &self.notes.len() } </p>
                    </div>
                </div>

                <div id="moodgrid">
                   { self.readings.iter().map(|r| render_bar(r.get())).collect::<Html>() }
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
