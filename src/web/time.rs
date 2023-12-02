use wasm_bindgen::JsValue;

pub fn formatted_js_date(epoch_millis_utc: u64) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(epoch_millis_utc as f64));

    let out = date.to_locale_time_string("en-US")
            .as_string()
            .unwrap_or_default();
    // pad string to 11 characters, padding on left
    format!("{:11}", out)    
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}
