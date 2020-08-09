use crate::{MoodReading, TextSubmission};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const SLEEP_KEY: &str = "sleep";

/// Yew wrapper around (in our case, local) storage.
/// See https://github.com/yewstack/yew/issues/1287#issuecomment-671043231
pub struct YewRepo {
    storage: StorageService,
}

impl YewRepo {
    pub fn save_mood_readings(&mut self, all: &Vec<MoodReading>) -> Result<(), SaveErr> {
        let value = Json(all);
        Ok(self.storage.store(MOOD_READINGS_KEY, value))
    }

    pub fn save_notes(&mut self, all: &Vec<TextSubmission>) -> Result<(), SaveErr> {
        let value = Json(all);
        Ok(self.storage.store(NOTES_KEY, value))
    }

    pub fn save_sleep(&mut self, all: &Vec<TextSubmission>) -> Result<(), SaveErr> {
        let value = Json(all);
        Ok(self.storage.store(SLEEP_KEY, value))
    }

    pub fn load_mood_readings(&self) -> Result<Vec<MoodReading>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(MOOD_READINGS_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }
    pub fn load_notes(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(NOTES_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }
    pub fn load_sleep(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(SLEEP_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }

    pub fn new() -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage }
    }
}

#[derive(Debug)]
pub struct SaveErr;
#[derive(Debug)]
pub struct LoadErr;
