use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum ExpressionType {
    gain,
    pan,
    transpose,
    timbre,
    formant,
    pressure,

    // MIDI
    channelController,
    channelPressure,
    polyPressure,
    pitchBend,
    programChange,
}
