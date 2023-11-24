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
    destination: Channel,
    #[serde(rename = "@type")]
    send_type: SendType,
}
