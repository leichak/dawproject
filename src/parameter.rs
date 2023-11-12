use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Parameter {
    #[serde(rename = "@parameterID")]
    parameter_id: i32,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
}
