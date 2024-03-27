use crate::add_one_get;
use serde::Deserialize;
use serde::Serialize;

use super::clip::Clip;

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clips {
    // Extends timeline
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
    // End of extension
    #[serde(rename = "$value")]
    pub clips: Option<Vec<Clip>>,
}

impl Clips {
    pub fn new_test(clips: Vec<Clip>) -> Self {
        Clips {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            clips: Some(clips),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            clips: Some(vec![]),
        }
    }
}
