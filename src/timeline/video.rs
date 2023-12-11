use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;
use crate::file_reference::FileReference;

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    // Extends media file
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
    #[serde(rename = "File")]
    files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    duration: Option<f64>,
    // End of extension
    // End of extension
    #[serde(rename = "@algorithm")]
    algorithm: Option<String>,
    #[serde(rename = "@channels")]
    channels: Option<i32>,
    #[serde(rename = "@sampleRate")]
    sample_rate: Option<i32>,
}

impl Video {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            files_sequence: None,
            duration: None,
            algorithm: None,
            channels: None,
            sample_rate: None,
        }
    }
}
