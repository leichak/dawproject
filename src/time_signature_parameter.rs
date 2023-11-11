#[derive(serde::Deserialize)]
pub struct TimeSignatureParameter {
    #[serde(rename = "@parameterID")]
    parameter_id: i32,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@numerator")]
    numerator: i32,
    #[serde(rename = "@denominator")]
    denominator: i32,
}
