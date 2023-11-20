use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BoolPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Vec<String>,
    // Extension ends
    #[serde(rename = "@value")]
    bool_point_sequence: Vec<bool>,
}
