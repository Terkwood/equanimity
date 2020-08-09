use crate::{MoodReading, TextSubmission};
use web_sys::window;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const SLEEP_KEY: &str = "sleep";

pub trait Repo {
    fn save_mood_readings(&mut self, all: &Vec<MoodReading>) -> Result<(), SaveErr>;
    fn load_mood_readings(&self) -> Result<Vec<MoodReading>, LoadErr>;
    fn save_notes(&mut self, all: &[TextSubmission]) -> Result<(), SaveErr>;
    fn load_notes(&self) -> Result<Vec<TextSubmission>, LoadErr>;
    fn save_sleep(&mut self, all: &[TextSubmission]) -> Result<(), SaveErr>;
    fn load_sleep(&self) -> Result<Vec<TextSubmission>, LoadErr>;
}

/// Yew wrapper around (in our case, local) storage.
/// See https://github.com/yewstack/yew/issues/1287#issuecomment-671043231
pub struct YewRepo {
    storage: StorageService,
}

impl Repo for YewRepo {
    fn save_mood_readings(&mut self, all: &Vec<MoodReading>) -> Result<(), SaveErr> {
        let value = Json(all);
        Ok(self.storage.store(MOOD_READINGS_KEY, value))
    }

    fn save_notes(&mut self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        todo!()
    }
    fn save_sleep(&mut self, all: &[TextSubmission]) -> Result<(), SaveErr> {
        todo!()
    }
    fn load_mood_readings(&self) -> Result<Vec<MoodReading>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(MOOD_READINGS_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }
    fn load_notes(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(NOTES_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }
    fn load_sleep(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(SLEEP_KEY) {
                restored_model
            } else {
                Vec::new()
            },
        )
    }
}

impl YewRepo {
    pub fn new() -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage }
    }
}
// TODO DELETE
/*
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
    fn load_mood_readings(&self) -> Result<Vec<MoodReading>, LoadErr> {
        todo!()
    }
    fn load_notes(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        todo!()
    }
    fn load_sleep(&self) -> Result<Vec<TextSubmission>, LoadErr> {
        todo!()
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
}*/

#[derive(Debug)]
pub struct SaveErr;
#[derive(Debug)]
pub struct LoadErr;
