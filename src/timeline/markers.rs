use crate::id_xml;
use crate::timeline::marker::Marker;
use crate::timeline::time_unit::TimeUnit;
use crate::track::Track;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Markers {
    // Extends Timeline
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@track")]
    track: Option<Track>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension ends
    #[serde(rename = "Marker")]
    markers: Option<Vec<Marker>>,
}

impl Markers {
    pub fn new_empty() -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id{}", id_xml.to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            markers: None,
        }
    }
}
