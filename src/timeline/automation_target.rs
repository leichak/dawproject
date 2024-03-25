use crate::expression_type::ExpressionType;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct AutomationTarget {
    #[serde(rename = "@parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "@expression")]
    pub expression: Option<ExpressionType>,
    #[serde(rename = "@channel")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    pub key: Option<i32>,
    #[serde(rename = "@controller")]
    pub controller: Option<i32>,
}

impl AutomationTarget {
    pub fn new_empty() -> Self {
        Self {
            parameter: None,
            expression: None,
            channel: None,
            key: None,
            controller: None,
        }
    }
}
