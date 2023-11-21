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
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@track")]
    track: Track,
    #[serde(rename = "@timeUnit")]
    time_unit: TimeUnit,
    // End of extension
    #[serde(default)]
    clips: Vec<Clip>,
}
