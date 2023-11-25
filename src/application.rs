use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: String,
}
