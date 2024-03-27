use super::timeline::TimeLine;
use super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit};
use serde::Deserialize;
use serde::Serialize;

type ClipSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clip {
    // #Extends nameable
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    // End of extension
    #[serde(rename = "$value")]
    pub notes_sequence_choice: Option<ClipSequenceChoice>,
    #[serde(rename = "@time")]
    pub time: f64,
    #[serde(rename = "@duration")]
    pub duration: Option<f64>,
    #[serde(rename = "@contentTimeUnit")]
    pub content_time_unit: Option<TimeUnit>,
    #[serde(rename = "@playStart")]
    pub play_start: Option<f64>,
    #[serde(rename = "@playStop")]
    pub play_stop: Option<f64>,
    #[serde(rename = "@loopStart")]
    pub loop_start: Option<f64>,
    #[serde(rename = "@loopEnd")]
    pub loop_end: Option<f64>,
    #[serde(rename = "@fadeTimeUnit")]
    pub fade_time_unit: Option<TimeUnit>,
    #[serde(rename = "@fadeInTime")]
    pub fade_in_time: Option<f64>,
    #[serde(rename = "@fadeOutTime")]
    pub fade_out_time: Option<f64>,
    #[serde(rename = "@content")]
    pub content: Option<TimeLine>, // in java only methods of super class are accesible so create enum that performs upcasting
    #[serde(rename = "@reference")]
    pub reference: Option<String>,
}

impl Clip {
    pub fn new_test(content: TimeLine, time: f64, duration: f64) -> Self {
        Clip {
            name: None,
            color: None,
            comment: None,
            notes_sequence_choice: None,
            time: time,
            duration: Some(duration),
            content_time_unit: None,
            play_start: None,
            play_stop: None,
            loop_start: None,
            loop_end: None,
            fade_time_unit: None,
            fade_in_time: None,
            fade_out_time: None,
            content: None,
            reference: None,
        }
    }

    pub fn new_empty() -> Self {
        Clip {
            name: None,
            color: None,
            comment: None,
            notes_sequence_choice: None,
            time: 0.0,
            duration: None,
            content_time_unit: None,
            play_start: None,
            play_stop: None,
            loop_start: None,
            loop_end: None,
            fade_time_unit: None,
            fade_in_time: None,
            fade_out_time: None,
            content: None,
            reference: None,
        }
    }
}
