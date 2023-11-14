use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum Unit {
    #[serde(rename = "@linear")]
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
