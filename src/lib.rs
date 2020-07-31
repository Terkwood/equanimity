#![recursion_limit = "1024"]
use chrono::prelude::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    readings: Vec<Reading>,
}

#[derive(Copy, Clone, Debug)]
struct Reading {
    value: i8,
}

const MIN_READING: i8 = -3;
const MAX_READING: i8 = 3;
impl Reading {
    pub fn new(value: i8) -> Reading {
        if value < MIN_READING {
            Reading { value: MIN_READING }
        } else if value > MAX_READING {
            Reading { value: MAX_READING }
        } else {
            Reading { value }
        }
    }
    pub fn get(self) -> i8 {
        self.value
    }
}

fn render_value(value: i8) -> Html {
    html! {
        <>
            <div class="hot3"></div>
            <div class="hot2"></div>
            <div class="hot1"></div>
            <div class="neutral"></div>
            <div class="nocolor"></div>
            <div class="nocolor"></div>
            <div class="nocolor"></div>
        </>
    }
}

fn class_from(value: i8, position: i8) -> String {
    if position == 0 {
        "neutral".to_string()
    } else if position > 0 && value >= position {
        format!("hot{}", position)
    } else if position < 0 && value <= position {
        format!("cold{}", position)
    } else {
        "nocolor".to_string()
    }
}

enum Msg {
    AddReading(Reading),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            readings: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddReading(r) => self.readings.push(r),
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
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-3)))>{ "-3" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-2)))>{ "-2" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(-1)))>{ "-1" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(0)))>{ "0" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(1)))>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(2)))>{ "+2" }</button>
                <button onclick=self.link.callback(|_| Msg::AddReading(Reading::new(3)))>{ "+3" }</button>

                <p>{"There are "} { self.readings.len() } {" readings"}</p>

                <p>{ self.readings.iter().map(|r| r.get()).collect::<Html>() }</p>

                <div id="grid">
                    // day 1
                    <div class="hot3"></div>
                    <div class="hot2"></div>
                    <div class="hot1"></div>
                    <div class="neutral"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    // day 2
                    <div class="nocolor"></div>
                    <div class="hot2"></div>
                    <div class="hot1"></div>
                    <div class="neutral"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    // day 3
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="hot1"></div>
                    <div class="neutral"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    // day 4
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="neutral"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    // day 5
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="neutral"></div>
                    <div class="cold1"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    // day 6
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="neutral"></div>
                    <div class="cold1"></div>
                    <div class="cold2"></div>
                    <div class="nocolor"></div>
                    // day 7
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="nocolor"></div>
                    <div class="neutral"></div>
                    <div class="cold1"></div>
                    <div class="cold2"></div>
                    <div class="cold3"></div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
