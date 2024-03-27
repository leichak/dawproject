use super::UpcastTimeline;
use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp,
};
use crate::add_one_get;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WarpsSequenceEnum {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Warps {
    // Extends Timeline
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>, // attribute
    #[serde(rename = "@color")]
    pub color: Option<String>, // att
    #[serde(rename = "@comment")]
    pub comment: Option<String>, // att
    #[serde(rename = "@track")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
    // Extension ends
    #[serde(rename = "$value")]
    pub warps_sequence: Option<WarpsSequence>,
    #[serde(rename = "@contentTimeUnit")]
    pub content_time_unit: TimeUnit,
}

impl Warps {
    pub fn new_test(time_unit: TimeUnit) -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            warps_sequence: None,
            content_time_unit: time_unit,
        }
    }
}

impl UpcastTimeline for Warps {
    // this is to emulate upcasting
    fn upcast(&self) -> TimeLine {
        TimeLine {
            id: self.id.clone(),
            name: self.name.clone(),
            color: self.color.clone(),
            comment: self.comment.clone(),
            track: self.track.clone(),
            time_unit: self.time_unit.clone(),
        }
    }
}
