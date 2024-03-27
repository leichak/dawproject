use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;

use super::{note::Note, time_unit::TimeUnit};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Notes {
    // Extends timeline
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>, // attribute
    #[serde(rename = "@color")]
    pub color: Option<String>, // att
    #[serde(rename = "@comment")]
    pub comment: Option<String>, // att
    #[serde(rename = "@track")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
    // Extension finishes
    #[serde(rename = "$value")]
    pub notes_sequence: Option<Vec<Note>>,
}

impl Notes {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            notes_sequence: Some(vec![]),
        }
    }
}
