use serde::Deserialize;

use super::clip::Clip;
use crate::track::Track;

use super::time_unit::TimeUnit;

#[derive(Deserialize, Debug)]
pub struct Clips {
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
    time_unit: Option<TimeUnit>,
    // End of extension
    #[serde(rename = "$value")]
    clips: Vec<Clip>,
}
