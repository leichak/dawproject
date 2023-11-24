use crate::expression_type::ExpressionType;

use serde::{Deserialize};

/*
 * <p>Defines the target of automation or expression, usually used within a Points element.</p>
 *
 * <p>Either it points directly to a parameter or an expression, and in the expression case
 * it can either be monophonic (such as MIDI CCs) or per-note/polyphonic (such as poly pressure)</p>
 */

#[derive(Deserialize, Debug)]
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
