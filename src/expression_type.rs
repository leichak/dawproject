use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExpressionTypeEnum {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpressionType {
    #[serde(rename = "$value")]
    pub expression_type: Vec<ExpressionTypeEnum>,
}

impl ExpressionType {
    pub fn new_test() -> Self {
        Self {
            expression_type: vec![],
        }
    }
}
