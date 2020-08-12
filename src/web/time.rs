use chrono::prelude::*;

pub fn local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
    todo!()
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}

const JS_CHRONO_OFFSET_COEFF: u64 = 10;
fn local_offset_seconds() -> u64 {
    js_sys::Date::new_0().get_timezone_offset() as u64 * JS_CHRONO_OFFSET_COEFF
}