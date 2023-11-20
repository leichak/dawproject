use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Marker {
    // Derives after nameable
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    #[serde(rename = "@time")]
    time: f64,
}
