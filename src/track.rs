use crate::content_type::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Track {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "$value")]
    contentType: Vec<ContentType>,
}
