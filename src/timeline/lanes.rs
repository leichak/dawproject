use crate::track::Track;

use serde::{Deserialize, Serialize};

use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warps::Warps,
};

#[derive(Deserialize, Debug)]
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

/* Lanes representing nested content. */
#[derive(Deserialize, Debug)]
pub struct Lanes {
    // Extends timeline
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
    // Extension finishes
    #[serde(rename = "$value")]
    lanes_sequence: Option<LanesSequenceChoice>,
}
