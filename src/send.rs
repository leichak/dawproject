use serde::{Deserialize, Serialize};

use crate::{channel::Channel, id_xml, real_parameter::RealParameter, send_type::SendType};
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
    pub fn new_empty() -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id_{}", id_xml.to_string())),
            volume: None,
            pan: None,
            destination: None,
            send_type: None,
        }
    }
}
