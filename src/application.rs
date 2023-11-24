use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Application {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}
