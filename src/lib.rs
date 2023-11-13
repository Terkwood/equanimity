#![recursion_limit = "1024"]
extern crate serde_derive;
extern crate serde_json;

mod moods;
mod repo;
mod web;
mod pips;

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web::time::utc_now;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct TextSubmission {
    pub epoch_millis: u64,
    pub value: String,
}

impl TextSubmission {
    pub fn new(value: String) -> Self {
        TextSubmission {
            value,
            epoch_millis: utc_now(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextType {
    Sleep,
    Meds,
    Notes,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoodReading {
    pub epoch_millis: u64,
    pub value: i8,
}

const MIN_READING: i8 = -3;
const MAX_READING: i8 = 3;
impl MoodReading {
    pub fn new(value: i8) -> MoodReading {
        let epoch_millis = utc_now();
        if value < MIN_READING {
            MoodReading {
                value: MIN_READING,
                epoch_millis,
            }
        } else if value > MAX_READING {
            MoodReading {
                value: MAX_READING,
                epoch_millis,
            }
        } else {
            MoodReading {
                value,
                epoch_millis,
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<web::Root>::new().mount_to_body();
}
