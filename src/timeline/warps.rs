use super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit, warp::Warp};
use crate::track::Track;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WarpsSequenceEnum {
    #[serde(rename = "$value")]
    holder: ArrangementTypeChoiceEnum,
    //Warp(Warp),
}

type WarpsSequence = Vec<WarpsSequenceEnum>;

#[derive(Deserialize, Debug)]
pub struct Warps {
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
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension ends
    #[serde(rename = "$value")]
    warps_sequence: WarpsSequence,
    #[serde(rename = "warp")]
    warp: Warp,
    #[serde(rename = "@contentTimeUnit")]
    content_time_unit: Option<TimeUnit>,
}
