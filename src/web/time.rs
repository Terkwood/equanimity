use chrono::prelude::*;

/// Note the bias to western timezone.
/// If this software were open-sourced, we'd want to make
/// this usable for individuals in the eastern hemisphere.
pub fn js_utc_datetime(epoch_millis_utc: u64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(epoch_millis_utc as i64).unwrap()
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}
