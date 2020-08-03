use crate::{MoodReading, TextSubmission};

const MOOD_READINGS_KEY: &str = "mood_readings";
const NOTES_KEY: &str = "notes";
const SLEEP_KEY: &str = "sleep";

fn save_mood_readings(all: &[MoodReading]) -> Result<(), SaveErr> {
    todo!()
}
fn load_mood_readings() -> Result<Vec<MoodReading>, LoadErr> {
    todo!()
}

fn save_notes(all: &[TextSubmission]) -> Result<(), SaveErr> {
    todo!()
}
fn load_notes() -> Result<Vec<TextSubmission>, LoadErr> {
    todo!()
}

fn save_sleep(all: &[TextSubmission]) -> Result<(), SaveErr> {
    todo!()
}
fn load_sleep() -> Result<Vec<TextSubmission>, LoadErr> {
    todo!()
}

struct SaveErr;
struct LoadErr;
