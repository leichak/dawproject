#[derive(serde::Deserialize, Debug)]
pub struct TimeSignatureParameter {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
    #[serde(rename = "@numerator")]
    numerator: i32,
    #[serde(rename = "@denominator")]
    denominator: i32,
}
