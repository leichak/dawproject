use serde::{Deserialize, Serialize};

use crate::add_one_get;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lane {
    // Extends referenceable / nameable
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
        Self {
            id: Some(format!("id_{}", add_one_get())),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
        }
    }
}
