use chrono::prelude::*;

pub fn local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
    todo!()
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}