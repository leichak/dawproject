use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileReference {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@external")]
    pub external: Option<bool>,
}

impl FileReference {
    pub fn new(path: String) -> Self {
        Self {
            path,
            external: None,
        }
    }
}
