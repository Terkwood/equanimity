use super::logs::Logs;
use crate::*;
use yew::{prelude::*, virtual_dom::VNode};
use yew_export_button::{export_button, ButtonOpts};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/Terkwood/equanimity";

const EXPORT_BUTTON_CSS_ID: &str = "export-button";
const EXPORT_LINK_CSS_CLASS: &str = "fancy-button thick";
const EXPORT_FILE_PREFIX: &str = "equanimity";

pub fn section(callback: Callback<MouseEvent>, ctx: &yew::Context<Logs>) -> Html {
    let export_button: VNode = export_button(
        &ctx.props().storage_state,
        ButtonOpts {
            a_class: EXPORT_LINK_CSS_CLASS.to_string(),
            button_id: EXPORT_BUTTON_CSS_ID.to_string(),
            file_prefix: EXPORT_FILE_PREFIX.to_string(),
            utc_millis: utc_now(),
        },
    );

    html! {
        <div id="about">
            <h1>{ "About" }</h1>
            <p>{ "EQUANIMITY helps you track mood variations." }</p>
            <p>{ "EQUANIMITY is designed with privacy in mind.  Your data will never be transmitted to a third party.  Data is kept in browser local storage, unencypted.  KEEP YOUR DATA SAFE: make sure there is no malware on your system!" }</p>
            <p>{ format!("This is version {}.", VERSION) }</p>
            <h2>{ "Source Code" }</h2>
            <p>{ "The source code is available under MIT license." }</p>
            <p><a href={REPO_URL}>{ REPO_URL }</a></p>

            <div class="center">
                {  export_button }
            </div>

            <button
                class="fancy-button thick"
                role="button"
                onclick={callback}>
                { "OK" }
            </button>
        </div>
    }
}
