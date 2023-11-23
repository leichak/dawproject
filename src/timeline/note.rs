use super::lanes::ArrangementTypeChoiceEnum;
use serde::Deserialize;

type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Deserialize, Debug)]
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
