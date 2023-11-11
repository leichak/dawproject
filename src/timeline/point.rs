/*A single automation point (abstract class). */
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Point {
    pub time: f64,
}
