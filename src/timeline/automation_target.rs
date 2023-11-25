use crate::expression_type::ExpressionType;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct AutomationTarget {
    #[serde(rename = "@parameter")]
    parameter: Option<String>,
    #[serde(rename = "@expression")]
    expression: Option<ExpressionType>,
    #[serde(rename = "@channel")]
    channel: Option<i32>,
    #[serde(rename = "@key")]
    key: Option<i32>,
    #[serde(rename = "@controller")]
    controller: Option<i32>,
}
