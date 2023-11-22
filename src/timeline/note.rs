use super::lanes::ArrangementTypeChoiceEnum;
use serde::Deserialize;

type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Deserialize, Debug)]
pub(crate) struct Note {
    #[serde(rename = "$value")]
    notes_sequence_choice: Option<NoteSequenceChoice>,
    #[serde(rename = "@time")]
    time: String,
    #[serde(rename = "@duration")]
    duration: String,
    #[serde(rename = "@channel")]
    channel: i32,
    #[serde(rename = "@key")]
    key: i32,
    #[serde(rename = "@vel")]
    vel: Option<String>,
    #[serde(rename = "@rel")]
    rel: Option<String>,
}
