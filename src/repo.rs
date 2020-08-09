use crate::{MoodReading, TextSubmission};
use web_sys::window;
use yew::services::storage::{Area, StorageService};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const SLEEP_KEY: &str = "sleep";

pub trait Repo {
    fn save_mood_readings(&self, all: &[MoodReading]) -> Result<(), SaveErr>;
    fn save_notes(&self, all: &[TextSubmission]) -> Result<(), SaveErr>;
    fn save_sleep(&self, all: &[TextSubmission]) -> Result<(), SaveErr>;
}

pub struct YewRepo {
    storage: StorageService,
}

impl Repo for YewRepo {
    fn save_mood_readings(&self, all: &[MoodReading]) -> Result<(), SaveErr> {
        todo!()
    }
    fn save_notes(&self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        todo!()
    }
    fn save_sleep(&self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        todo!()
    }
}

impl YewRepo {
    pub fn new() -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage }
    }
}

pub struct WebSysRepo;
impl Repo for WebSysRepo {
    fn save_mood_readings(&self, all: &[MoodReading]) -> Result<(), SaveErr> {
        if let Ok(data) = serde_json::to_string(all) {
            Self::save(MOOD_READINGS_KEY, &data)
        } else {
            Err(SaveErr)
        }
    }
    fn save_notes(&self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        if let Ok(data) = serde_json::to_string(all) {
            Self::save(NOTES_KEY, &data)
        } else {
            Err(SaveErr)
        }
    }
    fn save_sleep(&self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        if let Ok(data) = serde_json::to_string(all) {
            Self::save(SLEEP_KEY, &data)
        } else {
            Err(SaveErr)
        }
    }
}

impl WebSysRepo {
    fn save(key: &str, data: &str) -> Result<(), SaveErr> {
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
}

#[derive(Debug)]
pub struct SaveErr;
