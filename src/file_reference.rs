use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileReference {
    #[serde(rename = "@path")]
    path: String,
    #[serde(rename = "@external")]
    external: Option<bool>,
}
