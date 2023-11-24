use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BoolParameter {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
    #[serde(rename = "@value")]
    value: Option<bool>,
}
