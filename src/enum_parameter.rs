use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnumParameter {
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
    #[serde(rename = "@count")]
    count: i32,
    #[serde(rename = "@labels")]
    labels: Vec<String>,
    #[serde(rename = "@value")]
    value: Option<i32>,
}
