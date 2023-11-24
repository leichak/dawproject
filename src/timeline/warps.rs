use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp,
};


use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
