use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum Unit {
    linear,
    normalized,
    percent,
    decibel,
    hertz,
    semitones,
    seconds,
    beats,
    bpm,
}
