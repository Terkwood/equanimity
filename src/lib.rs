#![recursion_limit = "1024"]
extern crate serde_derive;
extern crate serde_json;

use chrono::{TimeZone, Utc};
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod repo;
mod web;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextSubmission {
    pub value: String,
    pub _epoch_millis: u64,
}

impl TextSubmission {
    pub fn new(value: String) -> Self {
        TextSubmission {
            value,
            _epoch_millis: now(),
        }
    }
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoodReading {
    pub epoch_millis: u64,
    pub value: i8,
}

fn now() -> u64 {
    js_sys::Date::now() as u64
}
const MIN_READING: i8 = -3;
const MAX_READING: i8 = 3;
impl MoodReading {
    pub fn new(value: i8) -> MoodReading {
        let epoch_millis = now_rand();
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

// TODO REMOVE THIS HACK
fn now_rand() -> u64 {
    let rn = now();
    let rrr = rn % 20;
    now() - rrr * 86400000
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<web::Model>::new().mount_to_body();
}
