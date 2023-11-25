use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Application {
    pub fn new_empty() -> Self {
        Application {
            name: "".to_string(),
            version: "".to_string(),
        }
    }
}
