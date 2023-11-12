use super::super::unit::Unit;
use super::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Points {
    #[serde(rename = "@points")]
    #[serde(default)]
    points: Vec<Point>, //The contained points. They should all be of the same type and match the target parameter. */
    #[serde(rename = "@unit")]
    unit: Unit,
}
