use chrono::prelude::*;

pub fn js_utc_datetime(epoch_millis_utc: u64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(epoch_millis_utc as i64).unwrap()
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}
