
use crate::{channel::Channel, real_parameter::RealParameter, send_type::SendType};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Send {
    // Extends referenceable
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "Volume")]
    volume: RealParameter, // required
    #[serde(rename = "Pan")]
    pan: Option<RealParameter>,
    #[serde(rename = "@destination")]
    destination: Option<Channel>,
    #[serde(rename = "@type")]
    send_type: SendType, // post when creating / just assign
}

impl Send {}
