use serde::Deserialize;

use super::{note::Note, time_unit::TimeUnit};
use crate::track::Track;

#[derive(Deserialize, Debug)]
pub struct Notes {
    // Extends timeline
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@track")]
    track: Track,
    #[serde(rename = "@timeUnit")]
    timeUnit: TimeUnit,
    // Extension finishes
    #[serde(default)]
    notes_sequence: Vec<Note>,
}
