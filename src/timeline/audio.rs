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
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@track")]
    track: Track,
    #[serde(rename = "@timeUnit")]
    timeUnit: TimeUnit,
    #[serde(rename = "$value")]
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
