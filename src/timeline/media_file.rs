use super::time_unit::TimeUnit;
use crate::{file_reference::FileReference, track::Track};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MediaFile {
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
    // End of extension
    #[serde(rename = "$value")]
    files_sequence: Vec<FileReference>,
    #[serde(rename = "@duration")]
    duration: f64,
}
