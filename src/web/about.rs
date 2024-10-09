use std::{future::IntoFuture, str::FromStr};

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

pub async fn on_click_import(e: web_sys::MouseEvent) -> Result<JsValue, JsValue> {
    let import_p = web_sys::window()
        .expect("no global window")
        .show_open_file_picker();
    match import_p {
        Ok(promise) => {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
            let deser: Result<StorageState, _> = result.into_serde();
            match deser {
                Err(_e) => console::error_1(&"deser error".into()),
                Ok(_) => console::log_1(&"IT WORKED".into()),
            }
            unimplemented!()
        }
        Err(_j) => {
            web_sys::console::error_1(&"error import".into());
            Ok(js_sys::JsString::from_str("help").expect("string").into())
        }
    }
}