use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@parameterID")]
    parameter_id: Option<i32>,
}

impl Parameter {
    pub fn new() -> Self {
        id_xml = id_xml + 1;
        Self {
            id: Some("id" + id_xml.to_string()),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
        }
    }
}
