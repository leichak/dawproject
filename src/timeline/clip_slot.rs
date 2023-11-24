use serde::Deserialize;

use super::clip::Clip;

use super::time_unit::TimeUnit;

#[derive(Deserialize, Debug)]
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
