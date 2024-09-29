use crate::{MoodReading, QuickMedButton, TextSubmission, TextType};
use gloo::storage::LocalStorage;
use gloo_storage::{errors::StorageError, Storage};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const MEDS_KEY: &str = "meds";
const SLEEP_KEY: &str = "sleep";
const QUICK_MED_BUTTONS_KEY: &str = "quick_med_buttons";

pub fn save_quick_med_buttons(all: &Vec<QuickMedButton>) -> Result<(), StorageError> {
    LocalStorage::set(QUICK_MED_BUTTONS_KEY, all)
}

pub fn load_quick_med_buttons() -> Result<Vec<QuickMedButton>, LoadErr> {
    Ok(
        if let Ok(restored_model) = LocalStorage::get(QUICK_MED_BUTTONS_KEY) {
            restored_model
        } else {
            Vec::new()
        },
    )
}

pub fn save_mood_readings(all: &Vec<MoodReading>) -> Result<(), StorageError> {
    LocalStorage::set(MOOD_READINGS_KEY, all)
}

pub fn save_text(text_type: TextType, all: &Vec<TextSubmission>) -> Result<(), StorageError> {
    LocalStorage::set(text_key(text_type), all)
}

pub fn load_mood_readings() -> Result<Vec<MoodReading>, LoadErr> {
    Ok(
        if let Ok(restored_model) = LocalStorage::get(MOOD_READINGS_KEY) {
            restored_model
        } else {
            Vec::new()
        },
    )
}
pub fn load_text(text_type: TextType) -> Result<Vec<TextSubmission>, LoadErr> {
    Ok(
        if let Ok(restored_model) = LocalStorage::get(text_key(text_type)) {
            restored_model
        } else {
            Vec::new()
        },
    )
}

fn text_key(text_type: TextType) -> &'static str {
    match text_type {
        TextType::Sleep => SLEEP_KEY,
        TextType::Meds => MEDS_KEY,
        TextType::Notes => NOTES_KEY,
    }
}

#[derive(Debug)]
pub struct LoadErr;
