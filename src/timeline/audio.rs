use serde::Deserialize;
use serde::Serialize;

use super::time_unit::TimeUnit;
use super::timeline::TimeLine;
use super::UpcastTimeline;
use crate::add_one_get;
use crate::file_reference::FileReference;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Audio {
    // Extends media file
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
    #[serde(rename = "File")]
    pub files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    pub duration: Option<f64>,
    // End of extension
    #[serde(rename = "@algorithm")]
    pub algorithm: Option<String>,
    #[serde(rename = "@channels")]
    pub channels: i32,
    #[serde(rename = "@sampleRate")]
    pub sample_rate: i32,
}

impl Audio {
    pub fn new_test(relative_path: String, sample_rate: i32, channels: i32, duration: f64) -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
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

impl UpcastTimeline for Audio {
    fn upcast(&self) -> TimeLine {
        TimeLine {
            id: self.id.clone(),
            name: self.name.clone(),
            color: self.color.clone(),
            comment: self.comment.clone(),
            track: self.track.clone(),
            time_unit: self.time_unit.clone(),
        }
    }
}
