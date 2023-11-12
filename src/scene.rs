use serde::{Deserialize, Serialize};

use crate::timeline::timeline::TimeLine;

#[derive(Deserialize)]
pub struct Scene {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "TimeLine")]
    pub content: TimeLine,
}
