use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeSignatureParameter {
    // Extends parameter
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

impl TimeSignatureParameter {
    pub fn new_required(numerator: i32, denominator: i32) -> Self {
        Self {
            name: None,
            color: None,
            comment: None,
            id: Some(format!("id{}", add_one_get())),
            parameter_id: None,
            numerator,
            denominator,
        }
    }
}
