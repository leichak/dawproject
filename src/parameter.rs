use crate::add_one_get;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Parameter {
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
}

impl Parameter {
    pub fn new() -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
        }
    }
}
