use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Marker {
    // Extends nameable
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    // End of extension
    #[serde(rename = "@time")]
    time: Option<Vec<f64>>,
}
