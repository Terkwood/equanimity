use js_sys::Object;
use wasm_bindgen::JsValue;

pub fn formatted_js_date(epoch_millis_utc: u64) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(epoch_millis_utc as f64));

    format!(
        "{} {}",
        date.to_locale_date_string("en-US", &Object::new())
            .as_string()
            .unwrap_or_default(),
        date.to_locale_time_string("en-US")
            .as_string()
            .unwrap_or_default()
    )
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}
