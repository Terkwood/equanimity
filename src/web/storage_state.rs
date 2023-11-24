use crate::*;
#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct StorageState {
    pub mood_readings: Vec<MoodReading>,
    pub meds: Vec<TextSubmission>,
    pub sleep_entries: Vec<TextSubmission>,
    pub notes: Vec<TextSubmission>,
}

impl StorageState {
    pub fn load() -> Self {
        Self {
            mood_readings: repo::load_mood_readings().unwrap_or_default(),
            meds: repo::load_text(TextType::Meds).unwrap_or_default(),
            sleep_entries: repo::load_text(TextType::Sleep).unwrap_or_default(),
            notes: repo::load_text(TextType::Notes).unwrap_or_default(),
        }
    }
}
