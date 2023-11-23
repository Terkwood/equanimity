use crate::{MoodReading, TextSubmission, TextType};
//use yew::format::Json;
use gloo::storage::LocalStorage;
use gloo_storage::{errors::StorageError, Storage};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const MEDS_KEY: &str = "meds";
const SLEEP_KEY: &str = "sleep";

pub fn save_mood_readings(all: &Vec<MoodReading>) -> Result<(), StorageError> {
    LocalStorage::set(
        MOOD_READINGS_KEY,
        serde_json::to_string(all).expect("readings to string"),
    )
}

pub fn save_text(text_type: TextType, all: &Vec<TextSubmission>) -> Result<(), StorageError> {
    LocalStorage::set(
        text_key(text_type),
        serde_json::to_string(all).expect("text subs to string"),
    )
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
pub struct SaveErr;
#[derive(Debug)]
pub struct LoadErr;
