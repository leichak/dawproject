use serde::{Deserialize, Serialize};

use crate::add_one_get;

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
        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
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
