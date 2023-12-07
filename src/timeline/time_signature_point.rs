use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
enum TimeSignaturePointEnum {
    #[serde(rename = "@numerator")]
    Numerator(i32),
    #[serde(rename = "@denominator")]
    Denominator(i32),
}

type TimeSignaturePointSequence = Vec<TimeSignaturePointEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSignaturePoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    real_point_sequence: Option<TimeSignaturePointSequence>,
}

impl TimeSignaturePoint {
    pub fn new() -> Self {
        Self {
            time: None,
            real_point_sequence: None,
        }
    }
}
