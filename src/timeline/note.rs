use super::lanes::ArrangementTypeChoiceEnum;
use serde::Deserialize;
use serde::Serialize;

type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Note {
    #[serde(rename = "$value")]
    notes_sequence_choice: Option<NoteSequenceChoice>,
    #[serde(rename = "@time")]
    time: Option<String>,
    #[serde(rename = "@duration")]
    duration: Option<String>,
    #[serde(rename = "@channel")]
    channel: Option<i32>,
    #[serde(rename = "@key")]
    key: Option<i32>,
    #[serde(rename = "@vel")]
    vel: Option<String>,
    #[serde(rename = "@rel")]
    rel: Option<String>,
}

impl Note {
    pub fn new() -> Self {
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
