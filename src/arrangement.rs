use crate::timeline::lanes::Lanes;
use crate::timeline::markers::Markers;
use crate::timeline::points::Points;
use crate::timeline::timeline;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Arrangement {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "TimeSignatureAutomation")]
    time_signature_automation: Points,
    #[serde(rename = "TempoAutomation")]
    tempo_automation: Points,
    #[serde(rename = "Markers")]
    markers: Markers,
    #[serde(rename = "Lanes")]
    lanes: Lanes,
}
