use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IntegerPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    integer_point_sequence: Option<Vec<i32>>,
}
