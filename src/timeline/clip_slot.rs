use super::clip::Clip;
use crate::add_one_get;
use serde::Deserialize;
use serde::Serialize;

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClipSlot {
    // Extends timeline
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
    time_unit: Option<TimeUnit>,
    // End of extension
    #[serde(default)]
    clips: Option<Vec<Clip>>,
    #[serde(rename = "@hasStop")]
    has_stop: bool,
}

impl ClipSlot {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            clips: None,
            has_stop: false,
        }
    }
}
