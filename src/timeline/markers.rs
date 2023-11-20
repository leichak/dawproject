use crate::timeline::marker::Marker;
use crate::timeline::time_unit::TimeUnit;
use crate::track::Track;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Markers {
    // Derives after Timeline
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
    #[serde(default)]
    #[serde(rename = "Marker")]
    markers: Vec<Marker>,
}
