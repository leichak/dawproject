/*A single automation point (abstract class). */
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Point {
    #[serde(rename = "@time")]
    pub time: f64,
}
