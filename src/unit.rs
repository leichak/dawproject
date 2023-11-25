use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
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
