use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Marker {
    // Extends nameable
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    // End of extension
    #[serde(rename = "@time")]
    time: Option<Vec<f64>>,
}

impl Marker {
    pub fn new(time: f64, name: String) -> Self {
        Self {
            name: Some(name),
            color: None,
            comment: None,
            time: Some(vec![time]),
        }
    }
}
