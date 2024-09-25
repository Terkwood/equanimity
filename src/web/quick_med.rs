use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::Component;

pub struct LogQuickMed {
    pub choice: QuickMedChoice,
    pub current_time: NaiveDateTime,
}

pub struct QuickMedChoice {}