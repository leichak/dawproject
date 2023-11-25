use crate::timeline::lanes::Lanes;
use crate::timeline::markers::Markers;
use crate::timeline::points::Points;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
enum ArrangementSequenceEnum {
    Points(Points),
    Lanes(Lanes),
    Markers(Markers),
    TempoAutomation(Points),
}

type ArrangementSequence = Vec<ArrangementSequenceEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Arrangement {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "$value")]
    sequence: Option<ArrangementSequence>,
}

impl Arrangement {
    pub fn new_empty() -> Self {
        Arrangement {
            id: None,
            name: None,
            color: None,
            comment: None,
            sequence: None,
        }
    }
}
