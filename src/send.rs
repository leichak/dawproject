use crate::add_one_get;
use crate::{channel::Channel, real_parameter::RealParameter, send_type::SendType};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
struct Send {
    // Extends referenceable
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "Volume")]
    volume: RealParameter,
    #[serde(rename = "Pan")]
    pan: Option<RealParameter>,
    #[serde(rename = "@destination")]
    destination: Option<Channel>,
    #[serde(rename = "@type")]
    send_type: Option<SendType>,
}

impl Send {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
            volume: RealParameter::new_required(),
            pan: None,
            destination: None,
            send_type: None,
        }
    }
}
