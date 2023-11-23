use crate::interpolation::Interpolation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]

enum TimeSignaturePointEnum {
    #[serde(rename = "@numerator")]
    Numerator(i32),
    #[serde(rename = "@denominator")]
    Denominator(i32),
}

type TimeSignaturePointSequence = Vec<TimeSignaturePointEnum>;

#[derive(Deserialize, Debug)]
pub struct TimeSignaturePoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    real_point_sequence: Option<TimeSignaturePointSequence>,
}
