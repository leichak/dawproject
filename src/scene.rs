use serde::Deserialize;
use serde::Serialize;

use crate::timeline::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, timeline::TimeLine, video::Video, warps::Warps,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum SceneSequenceEnum {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Scene {
    // extends referenceable
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    // #[serde(rename = "$value")]
    // warps_sequence: Option<SceneSequenceEnum>,
    #[serde(rename = "Timeline")] // not sure its good but let it be
    content: Option<TimeLine>,
}

impl Scene {}
