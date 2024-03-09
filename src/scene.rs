use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;
use crate::timeline::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, timeline::TimeLine, video::Video, warps::Warps,
};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

impl Scene {
    pub fn new() -> Self {
        // Self {
        //     id: Some(format!("id_{}", add_one_get().to_string())),
        //     name: todo!(),
        //     color: todo!(),
        //     comment: todo!(),
        //     warps_sequence: todo!(),
        // }
    }
}
