use crate::{MoodReading, TextSubmission};
use web_sys::window;

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const SLEEP_KEY: &str = "sleep";

pub fn save(key: &str, data: &str) -> Result<(), SaveErr> {
    if let Some(w) = window() {
        if let Ok(Some(storage)) = w.local_storage() {
            storage.set_item(key, data).map_err(|_| SaveErr)
        } else {
            Err(SaveErr)
        }
    } else {
        Err(SaveErr)
    }
}

pub fn save_mood_readings(all: &[MoodReading]) -> Result<(), SaveErr> {
    if let Ok(data) = serde_json::to_string(all) {
        save(MOOD_READINGS_KEY, &data)
    } else {
        Err(SaveErr)
    }
}
pub fn load_mood_readings() -> Result<Vec<MoodReading>, LoadErr> {
    todo!()
}

pub fn save_notes(all: &[TextSubmission]) -> Result<(), SaveErr> {
    if let Ok(data) = serde_json::to_string(all) {
        save(NOTES_KEY, &data)
    } else {
        Err(SaveErr)
    }
}
pub fn load_notes() -> Result<Vec<TextSubmission>, LoadErr> {
    todo!()
}

pub fn save_sleep(all: &[TextSubmission]) -> Result<(), SaveErr> {
    if let Ok(data) = serde_json::to_string(all) {
        save(SLEEP_KEY, &data)
    } else {
        Err(SaveErr)
    }
}
fn load_sleep() -> Result<Vec<TextSubmission>, LoadErr> {
    todo!()
}

#[derive(Debug)]
pub struct SaveErr;
#[derive(Debug)]
pub struct LoadErr;
