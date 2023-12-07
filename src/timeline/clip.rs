use super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit};
use serde::Deserialize;
use serde::Serialize;

type ClipSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Clip {
    // #Extends nameable
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    // End of extension
    #[serde(rename = "$value")]
    notes_sequence_choice: Option<ClipSequenceChoice>,
    #[serde(rename = "@time")]
    time: f64,
    #[serde(rename = "@duration")]
    duration: Option<f64>,
    #[serde(rename = "@contentTimeUnit")]
    content_time_unit: Option<TimeUnit>,
    #[serde(rename = "@playStart")]
    play_start: Option<f64>,
    #[serde(rename = "@playStop")]
    play_stop: Option<f64>,
    #[serde(rename = "@loopStart")]
    loop_start: Option<f64>,
    #[serde(rename = "@loopEnd")]
    loop_end: Option<f64>,
    #[serde(rename = "@fadeTimeUnit")]
    fade_time_unit: Option<TimeUnit>,
    #[serde(rename = "@fadeInTime")]
    fade_in_time: Option<f64>,
    #[serde(rename = "@fadeOutTime")]
    fade_out_time: Option<f64>,
    #[serde(rename = "@reference")]
    reference: Option<String>,
}

impl Clip {
    pub fn new() -> Self {
        Self {
            name: None,
            color: None,
            comment: None,
            notes_sequence_choice: None,
            time: 0,
            duration: None,
            content_time_unit: None,
            play_start: None,
            play_stop: None,
            loop_start: None,
            loop_end: None,
            fade_time_unit: None,
            fade_in_time: None,
            fade_out_time: None,
            reference: None,
        }
    }
}
