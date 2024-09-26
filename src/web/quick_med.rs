use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::Component;

pub struct LogQuickMed {
    pub choice: QuickMedChoice,
    pub current_time: NaiveDateTime,
}

pub struct QuickMedMsg {}

pub struct QuickMedChoice {}
pub struct QuickMedProps {}

impl Component for LogQuickMed {
    type Message = QuickMedMsg;
    type Properties = QuickMedProps;
}