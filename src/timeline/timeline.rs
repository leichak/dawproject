use serde::Deserialize;
use serde::Serialize;

use crate::timeline::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeLine {
    // Extends referenceable
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
}
