use serde::{Deserialize, Serialize};

use crate::timeline::time_unit::TimeUnit;
use crate::track::Track;

#[derive(Deserialize, Debug)]
pub struct TimeLine {
    // Derives after referenceable
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
    timeUnit: TimeUnit,
}
