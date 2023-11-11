use super::super::unit::Unit;
use super::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Points {
    points: Vec<Point>, //The contained points. They should all be of the same type and match the target parameter. */
    unit: Unit,
}
