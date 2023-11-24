use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lane {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>, //
    #[serde(rename = "@comment")]
    comment: Option<String>,
}
