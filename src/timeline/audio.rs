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

impl Audio {
    pub fn new_test(relative_path: String, sample_rate: i32, channels: i32, duration: f64) -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            files_sequence: Some(vec![FileReference {
                path: relative_path,
                external: None,
            }]),
            duration: Some(duration),
            algorithm: None,
            channels: channels,
            sample_rate: sample_rate,
        }
    }
}
