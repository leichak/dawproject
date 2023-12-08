use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
    #[serde(rename = "@max")]
    max: Option<i32>,
    #[serde(rename = "@min")]
    min: Option<i32>,
    #[serde(rename = "@value")]
    value: Option<i32>,
}

impl BoolParameter {
    pub fn new() -> Self {
        id_xml += 1;
        Self {
            id: Some("id" + id_xml.to_string),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
            max: None,
            min: None,
            value: None,
        }
    }
}
