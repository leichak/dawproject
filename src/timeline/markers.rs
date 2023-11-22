use crate::timeline::marker::Marker;
use crate::timeline::time_unit::TimeUnit;
use crate::track::Track;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Markers {
    // Extends Timeline
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "@track")]
    track: Option<Track>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension ends
    #[serde(default)]
    #[serde(rename = "Marker")]
    markers: Vec<Marker>,
}
