use crate::unit::Unit;

#[derive(serde::Deserialize, Debug)]
pub struct RealParameter {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@parameterID")]
    parameter_id: i32,
    #[serde(rename = "@value")]
    value: f64,
    #[serde(rename = "@unit")]
    unit: Unit,
    #[serde(rename = "@min")]
    min: f64,
    #[serde(rename = "@max")]
    max: f64,
}
