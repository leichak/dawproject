use serde::{Deserialize, Serialize};

use crate::id_xml;

#[derive(Debug, Deserialize, Serialize)]
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

impl Lane {
    pub fn new() -> Self {
        id_xml = id_xml + 1;
        Self {
            id: Some("id" + id_xml.to_string()),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
        }
    }
}
