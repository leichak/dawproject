use crate::interpolation::Interpolation;
use serde::Deserialize;
use serde::Serialize;

// #[derive(Debug, Deserialize, Serialize)]
// enum RealPointEnum {
//     #[serde(rename = "@value")]
//     Value(String),
//     #[serde(rename = "@interpolation")]
//     Interpolation(Interpolation),
// }

// type RealPointSequence = Vec<RealPointEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct RealPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<f64>,
    // Extension ends
    #[serde(rename = "@value")]
    pub value: Option<f64>,
    #[serde(rename = "@interpolation")]
    pub interpolation: Option<Interpolation>,
}

impl RealPoint {
    pub fn new_empty() -> Self {
        Self {
            time: None,
            value: None,
            interpolation: None,
        }
    }
}
