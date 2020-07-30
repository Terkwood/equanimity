#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
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
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
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
