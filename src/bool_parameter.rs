use serde::{Deserialize, Serialize};

use crate::id_xml;

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
    #[serde(rename = "@value")]
    value: Option<bool>,
}

impl BoolParameter {
    pub fn new() -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id_{}", id_xml.to_string())),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
            parameter_id: todo!(),
            value: todo!(),
        }
    }
}
