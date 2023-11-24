use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum ExpressionTypeEnum {
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

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ExpressionType {
    #[serde(rename = "$value")]
    expression_type: Vec<ExpressionTypeEnum>,
}
