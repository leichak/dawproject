use crate::add_one_get;

use serde::Deserialize;
use serde::Serialize;

use crate::unit::Unit;

#[derive(Debug, Deserialize, Serialize)]
pub struct RealParameter {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
    #[serde(rename = "@value")]
    pub value: Option<f64>,
    #[serde(rename = "@unit")]
    unit: Unit,
    #[serde(rename = "@min")]
    min: Option<f64>,
    #[serde(rename = "@max")]
    max: Option<f64>,
}

impl RealParameter {
    pub fn new_test(unit: Unit) -> RealParameter {
        RealParameter {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
            value: None,
            unit: unit,
            min: None,
            max: None,
        }
    }
}
