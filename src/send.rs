use serde::{Deserialize, Serialize};

use crate::real_parameter::RealParameter;
#[derive(Deserialize, Serialize, Debug)]
struct Send {
    // Extends referenceable
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "Volume")]
    volume: Option<RealParameter>,
    #[serde(rename = "Pan")]
    pan: Option<RealParameter>,
    #[serde(rename = "@destination")]
    destination: Option<Channel>,
    #[serde(rename = "@type")]
    send_type: Option<SendType>,
}

impl Send {
    pub fn new() -> Self {
        Self {
            id: Some(format!("id_{}", id_xml.to_string())),
            volume: None,
            pan: None,
            destination: None,
            send_type: None,
        }
    }
}
