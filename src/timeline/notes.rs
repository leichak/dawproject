use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;

use super::{note::Note, time_unit::TimeUnit};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notes {
    // Extends timeline
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "@track")]
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension finishes
    #[serde(rename = "$value")]
    notes_sequence: Option<Vec<Note>>,
}

impl Notes {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            notes_sequence: None,
        }
    }
}
