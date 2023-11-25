use super::time_unit::TimeUnit;
use crate::file_reference::FileReference;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct MediaFile {
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
    // End of extension
    #[serde(rename = "File")]
    files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    duration: Option<f64>,
}
