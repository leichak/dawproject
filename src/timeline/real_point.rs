use crate::interpolation::Interpolation;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
enum RealPointEnum {
    #[serde(rename = "@value")]
    Value(String),
    #[serde(rename = "@interpolation")]
    Interpolation(Interpolation),
}

type RealPointSequence = Vec<RealPointEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct RealPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    real_point_sequence: Option<RealPointSequence>,
}

impl RealPoint {
    pub fn new() -> Self {
        Self {
            time: None,
            real_point_sequence: None,
        }
    }
}
