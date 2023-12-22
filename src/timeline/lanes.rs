use serde::Deserialize;
use serde::Serialize;

use crate::id_xml;

use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warps::Warps,
};

#[derive(Debug, Deserialize, Serialize)]
pub enum ArrangementTypeChoiceEnum {
    Timeline(TimeLine),
    Lanes(Lanes),
    Notes(Notes),
    Clips(Clips),
    ClipSlot(ClipSlot),
    markers(Markers),
    Warps(Warps),
    Audio(Audio),
    Video(Video),
    Points(Points),
}

type LanesSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Lanes {
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
    timeUnit: Option<TimeUnit>,
    // Extension finishes
    #[serde(rename = "$value")]
    lanes_sequence: Option<LanesSequenceChoice>,
}

impl Lanes {
    pub fn new_empty() -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id{}", id_xml.to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            lanes_sequence: None,
        }
    }
}
