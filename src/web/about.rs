use yew::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/Terkwood/equanimity";

pub fn section(callback: Callback<MouseEvent>) -> Html {
    html! {
        <div id="about">
            <h1>{ "About" }</h1>
            <p>{ "EQUANIMITY helps you track mood variations." }</p>
            <p>{ "EQUANIMITY is designed with privacy in mind.  Your data will never be transmitted to a third party.  Data is kept in browser local storage, unencypted.  KEEP YOUR DATA SAFE: make sure there is no malware on your system!" }</p>
            <p>{ format!("This is version {}.", VERSION) }</p>
            <h2>{ "Source Code" }</h2>
            <p>{ "The source code is available under MIT license." }</p>
            <p><a href=REPO_URL>{ REPO_URL }</a></p>
            <button
                class="fancy-button thick"
                role="button"
                onclick=callback>
                { "OK" }
            </button>
        </div>
    }
}
