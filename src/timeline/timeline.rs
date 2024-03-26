use serde::Deserialize;
use serde::Serialize;

use crate::add_one_get;
use crate::timeline::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeLine {
    // Extends referenceable
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@track")] // this is IDREF type
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
}

impl TimeLine {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
        }
    }
}
