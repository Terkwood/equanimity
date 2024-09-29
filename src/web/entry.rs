use crate::*;
use std::collections::HashMap;
use super::storage_state::*;
use chrono::NaiveDate;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Entry {
    Mood(MoodReading),
    Sleep(TextSubmission),
    Meds(TextSubmission),
    Note(TextSubmission),
}

impl Entry {
    pub fn timestamp(&self) -> u64 {
        match self {
            Entry::Mood(m) => m.epoch_millis,
            Entry::Sleep(t) => t.epoch_millis,
            Entry::Meds(m) => m.epoch_millis,
            Entry::Note(t) => t.epoch_millis,
        }
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}

pub fn derive_entries(storage_state: &StorageState) -> HashMap<NaiveDate, Vec<Entry>> {
    let mut entries: HashMap<NaiveDate, Vec<Entry>> = HashMap::new();
    for m in &storage_state.mood_readings {
        let d = entry_date(&Entry::Mood(m.clone()));
        if let Some(e) = entries.get_mut(&d) {
            e.push(Entry::Mood(m.clone()))
        } else {
            entries.insert(d, vec![Entry::Mood(m.clone())]);
        }
    }
    for s in &storage_state.sleep_entries {
        let d = entry_date(&Entry::Sleep(s.clone()));
        if let Some(e) = entries.get_mut(&d) {
            e.push(Entry::Sleep(s.clone()))
        } else {
            entries.insert(d, vec![Entry::Sleep(s.clone())]);
        }
    }
    for m in &storage_state.meds {
        let d = entry_date(&Entry::Meds(m.clone()));
        if let Some(e) = entries.get_mut(&d) {
            e.push(Entry::Meds(m.clone()))
        } else {
            entries.insert(d, vec![Entry::Meds(m.clone())]);
        }
    }
    for n in &storage_state.notes {
        let d = entry_date(&Entry::Note(n.clone()));
        if let Some(e) = entries.get_mut(&d) {
            e.push(Entry::Note(n.clone()))
        } else {
            entries.insert(d, vec![Entry::Note(n.clone())]);
        }
    }

    entries
}

pub fn entry_date(e: &Entry) -> NaiveDate {
    let date = js_sys::Date::new(&JsValue::from_f64(e.timestamp() as f64));

    NaiveDate::from_ymd_opt(
        date.get_full_year() as i32,
        date.get_month() as u32 + 1,
        date.get_date() as u32,
    )
    .unwrap()
}