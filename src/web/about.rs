use std::future::IntoFuture;

use super::logs::Logs;
use crate::*;
use futures::TryFutureExt;
use gloo::utils::format::JsValueSerdeExt;
use js_sys::Promise;
use web::storage_state::StorageState;
use web_sys::console;
use yew::{prelude::*, virtual_dom::VNode};
use yew_export_button::{export_button, ButtonOpts};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/Terkwood/equanimity";

const EXPORT_BUTTON_CSS_ID: &str = "export-button";
const EXPORT_LINK_CSS_CLASS: &str = "fancy-button thick";
const EXPORT_FILE_PREFIX: &str = "equanimity";

pub fn section(ok_callback: Callback<MouseEvent>, ctx: &yew::Context<Logs>) -> Html {
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
        <div>
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
                onclick={on_click_import}>
                { "Import 📥" }
            </button>


            <button
                class="fancy-button thick"
                role="button"
                onclick={ok_callback}>
                { "OK" }
            </button>
        </div>
    }
}


pub async fn on_click_import( e: web_sys::MouseEvent) -> Result<(), ()> {
    let import_p = web_sys::window()
        .expect("no global window")
        .show_open_file_picker();
    match import_p {
        Ok(promise) => {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
            let deser: Result<StorageState, _>  = result.into_serde();
            match deser {
                Err(e) => console::error_1(&"deser error".into()),
                Ok(_) => console::log_1(&"IT WORKED".into())
            }
            unimplemented!()
        }
        Err(_j) => {
            web_sys::console::error_1(&"error import".into());
            Ok(())
        }
    }
}