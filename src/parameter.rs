use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Parameter {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
}
