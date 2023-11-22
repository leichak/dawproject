use serde::Deserialize;

use crate::{file_reference::FileReference, track::Track};

use super::time_unit::TimeUnit;

#[derive(Deserialize, Debug)]
pub struct Audio {
    // Extends media file
    // Extends timeline
    #[serde(rename = "@id")]
    id: String,
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
    #[serde(rename = "File")]
    files_sequence: Vec<FileReference>,
    #[serde(rename = "@duration")]
    duration: f64,
    // End of extension
    #[serde(rename = "@algorithm")]
    algorithm: Option<String>,
    #[serde(rename = "@channels")]
    channels: i32,
    #[serde(rename = "@sampleRate")]
    sample_rate: i32,
}
