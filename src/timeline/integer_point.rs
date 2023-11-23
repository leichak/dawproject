use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IntegerPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    integer_point_sequence: Option<Vec<i32>>,
}
