use crate::unit::Unit;

#[derive(serde::Deserialize, Debug)]
pub struct RealParameter {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
    #[serde(rename = "@value")]
    value: f64,
    #[serde(rename = "@unit")]
    unit: Unit,
    #[serde(rename = "@min")]
    min: f64,
    #[serde(rename = "@max")]
    max: f64,
}
