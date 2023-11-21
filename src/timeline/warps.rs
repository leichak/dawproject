use super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit, warp::Warp};
use crate::track::Track;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum WarpsSequenceEnum {
    ArrangementTypeChoiceEnum(ArrangementTypeChoiceEnum),
    Warp(Warp),
}

type WarpsSequence = Vec<WarpsSequenceEnum>;

#[derive(Deserialize, Debug)]
pub struct Warps {
    // Extends Timeline
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
    // Extension ends
    #[serde(rename = "$value")]
    warps_sequence: WarpsSequence,
    #[serde(rename = "@contentTimeUnit")]
    content_time_unit: TimeUnit,
}
