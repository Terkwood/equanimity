use crate::{MoodReading, TextSubmission, TextType};
//use yew::format::Json;
use gloo::storage::LocalStorage;
// use yew StorageService


const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const MEDS_KEY: &str = "meds";
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

    pub fn save_text(
        &mut self,
        text_type: TextType,
        all: &Vec<TextSubmission>,
    ) -> Result<(), SaveErr> {
        let value = Json(all);
        Ok(self.storage.store(text_key(text_type), value))
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
    pub fn load_text(&self, text_type: TextType) -> Result<Vec<TextSubmission>, LoadErr> {
        Ok(
            if let Json(Ok(restored_model)) = self.storage.restore(text_key(text_type)) {
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

fn text_key(text_type: TextType) -> &'static str {
    match text_type {
        TextType::Sleep => SLEEP_KEY,
        TextType::Meds => MEDS_KEY,
        TextType::Notes => NOTES_KEY,
    }
}

#[derive(Debug)]
pub struct SaveErr;
#[derive(Debug)]
pub struct LoadErr;
