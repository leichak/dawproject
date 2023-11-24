use serde::{Deserialize};

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]

pub(crate) struct ExpressionType {
    #[serde(rename = "$value")]
    expression_type: Vec<ExpressionTypeEnum>,
}
