use crate::interpolation::Interpolation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]

enum RealPointEnum {
    #[serde(rename = "@value")]
    Value(String),
    #[serde(rename = "@interpolation")]
    Interpolation(Interpolation),
}

type RealPointSequence = Vec<RealPointEnum>;

#[derive(Deserialize, Debug)]
pub struct RealPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Vec<String>,
    // Extension ends
    #[serde(rename = "@value")]
    real_point_sequence: RealPointSequence,
}
