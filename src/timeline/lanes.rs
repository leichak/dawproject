use super::timeline::TimeLine;
use serde::{Deserialize, Serialize};

/* Lanes representing nested content. */
#[derive(Deserialize)]
pub struct Lanes {
    #[serde(rename = "@lanes")]
    lanes: Vec<TimeLine>,
}
