use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnumPoint {
    // Extends Point
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
    // Extension ends
    #[serde(rename = "$value")]
    enum_point_sequence: Option<Vec<i32>>,
}
