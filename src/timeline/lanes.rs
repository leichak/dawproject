use super::timeline::TimeLine;
use serde::{Deserialize, Serialize};

/* Lanes representing nested content. */
#[derive(Deserialize)]
pub struct Lanes {
    lanes: Vec<TimeLine>,
}
