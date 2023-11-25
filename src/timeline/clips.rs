use serde::Deserialize;
use serde::Serialize;

use super::clip::Clip;

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Clips {
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
    #[serde(rename = "$value")]
    clips: Option<Vec<Clip>>,
}
