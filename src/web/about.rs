use std::{future::IntoFuture, str::FromStr};

use crate::*;
use gloo::utils::format::JsValueSerdeExt;
use web::storage_state::StorageState;
use web_sys::console;
pub async fn on_click_import(e: web_sys::MouseEvent) -> Result<JsValue, JsValue> {
    let import_p = web_sys::window()
        .expect("no global window")
        .show_open_file_picker();
    match import_p {
        Ok(promise) => {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
            let deser: Result<TestJson, _> = result.into_serde();
            match deser {
                Err(_e) => { 
                    console::error_1(&"deser error".into());
                    Err(js_sys::JsString::from_str("It failed to deser").expect("js str").into())
                
                 },
                Ok(_) => { console::log_1(&"IT WORKED".into());
                    Ok(js_sys::JsString::from_str("Nicely done").expect("js str").into())
                },
            }
        }
        Err(_j) => {
            web_sys::console::error_1(&"error import".into());
            Ok(js_sys::JsString::from_str("help").expect("string").into())
        }
    }
}

#[derive(Deserialize)]
pub struct TestJson { pub test: String }