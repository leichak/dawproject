use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    Linear,
    Normalized,
    Percent,
    Decibel,
    Hertz,
    Semitones,
    Seconds,
    Beats,
    Bpm,
}
