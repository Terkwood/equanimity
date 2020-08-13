use crate::web::StorageState;
use yew::prelude::*;

pub fn button(storage_state: &StorageState) -> Html {
    let filename: String = todo!();
    if let Ok(href) = provide_data(storage_state) {
        html! { <button class="thick"><a href=href download=filename>{ "Export 💾" }</a></button> }
    } else {
        html! { <button class="thick">{ "Export N/A ⛔" }</button>}
    }
}

const FILE_TYPE: &str = "application/json";

fn provide_data(state: &StorageState) -> Result<String, ProvideDataErr> {
    if let Ok(ser) = serde_json::to_string(state) {
        let encoded: String = js_sys::encode_uri_component(&ser).into();

        Ok(format!("data:{};charset=utf-8,{}", FILE_TYPE, encoded))
    } else {
        Err(ProvideDataErr)
    }
}

struct ProvideDataErr;
