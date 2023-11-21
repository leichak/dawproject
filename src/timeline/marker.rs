use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Marker {
    // Extends nameable
    #[serde(rename = "@name")]
    name: String, // attribute
    #[serde(rename = "@color")]
    color: String, // att
    #[serde(rename = "@comment")]
    comment: String, // att
    // End of extension
    #[serde(rename = "@time")]
    time: Vec<f64>,
}
