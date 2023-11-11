use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Application {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}
