use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp,
};
use crate::add_one_get;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug, Serialize)]
enum WarpsSequenceEnum {
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
    Warp(Warp),
}

type WarpsSequence = Vec<WarpsSequenceEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Warps {
    // Extends Timeline
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
    // Extension ends
    #[serde(rename = "$value")]
    warps_sequence: Option<WarpsSequence>,
    #[serde(rename = "@contentTimeUnit")]
    content_time_unit: Option<TimeUnit>,
}

impl Warps {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            warps_sequence: None,
            content_time_unit: None,
        }
    }
}
