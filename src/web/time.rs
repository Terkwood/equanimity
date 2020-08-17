use chrono::prelude::*;

/// Note the bias to western timezone.
/// If this software were open-sourced, we'd want to make
/// this usable for individuals in the eastern hemisphere.
pub fn js_local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
    let offset = FixedOffset::west(local_offset_seconds());
    Utc.timestamp_millis(epoch_millis_utc as i64)
        .with_timezone(&offset)
}

pub fn utc_now() -> u64 {
    js_sys::Date::now() as u64
}

const JS_CHRONO_OFFSET_COEFF: i32 = 60;
fn local_offset_seconds() -> i32 {
    js_sys::Date::new_0().get_timezone_offset() as i32 * JS_CHRONO_OFFSET_COEFF
}
