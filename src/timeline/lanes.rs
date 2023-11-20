use super::timeline::TimeLine;
use serde::{Deserialize, Serialize};

/* Lanes representing nested content. */
#[derive(Deserialize, Debug)]
pub struct Lanes {
    #[serde(rename = "@lanes")]
    lanes: Vec<TimeLine>,
}
