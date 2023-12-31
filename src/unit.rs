use serde::{Deserialize};

#[derive(Deserialize, Debug)]
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
