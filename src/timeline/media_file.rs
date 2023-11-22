use super::time_unit::TimeUnit;
use crate::{file_reference::FileReference, track::Track};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MediaFile {
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
    // End of extension
    #[serde(rename = "File")]
    files_sequence: Vec<FileReference>,
    #[serde(rename = "@duration")]
    duration: f64,
}
