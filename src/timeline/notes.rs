use serde::Deserialize;

use super::{note::Note, time_unit::TimeUnit};
use crate::track::Track;

#[derive(Deserialize, Debug)]
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
