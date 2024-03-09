use serde::Deserialize;
use serde::Serialize;

use super::time_unit::TimeUnit;
use crate::add_one_get;
use crate::file_reference::FileReference;

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {
    // Extends media file
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@track")]
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    #[serde(rename = "File")]
    files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    duration: Option<f64>,
    // End of extension
    #[serde(rename = "@algorithm")]
    algorithm: Option<String>,
    #[serde(rename = "@channels")]
    channels: i32,
    #[serde(rename = "@sampleRate")]
    sample_rate: i32,
}

impl Audio {}
