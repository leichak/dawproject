use crate::unit::Unit;

#[derive(serde::Deserialize)]
pub struct RealParameter {
    #[serde(rename = "@parameterID")]
    parameter_id: i32,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@value")]
    value: f64,
    #[serde(rename = "@unit")]
    unit: Unit,
    #[serde(rename = "@min")]
    min: f64,
    #[serde(rename = "@max")]
    max: f64,
}
