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

    pub fn new_name_ver(name: String, version: f64) -> Self {
        Application {
            name: name,
            version: version.to_string(),
        }
    }
}
