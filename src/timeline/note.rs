use super::lanes::ArrangementTypeChoiceEnum;
use serde::Deserialize;
use serde::Serialize;

type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Note {
    #[serde(rename = "$value")]
    pub notes_sequence_choice: Option<NoteSequenceChoice>,
    #[serde(rename = "@time")]
    pub time: Option<f64>,
    #[serde(rename = "@duration")]
    pub duration: Option<f64>,
    #[serde(rename = "@channel")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    pub key: Option<i32>,
    #[serde(rename = "@velocity")]
    pub vel: Option<f64>,
    #[serde(rename = "@releaseVelocity")]
    pub rel: Option<f64>,
}

impl Note {
    pub fn new_empty() -> Self {
        Self {
            notes_sequence_choice: None,
            time: None,
            duration: None,
            channel: None,
            key: None,
            vel: None,
            rel: None,
        }
    }
}
