use serde::Deserialize;
use serde::Serialize;

use crate::id_xml;

#[derive(Debug, Deserialize, Serialize)]
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
    count: Option<i32>,
    #[serde(rename = "@labels")]
    labels: Option<Vec<String>>,
    #[serde(rename = "@value")]
    value: Option<i32>,
}

impl EnumParameter {
    pub fn new() -> Self {
        Self {
            id: Some(format!("id_{}", id_xml.to_string())),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
            count: None,
            labels: None,
            value: None,
        }
    }
}
