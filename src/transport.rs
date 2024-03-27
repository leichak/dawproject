use crate::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TransportSequence {
    Tempo(RealParameter),
    TimeSignature(TimeSignatureParameter),
}

type TransportSequenceVec = Vec<TransportSequence>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transport {
    #[serde(rename = "$value")]
    pub sequence: TransportSequenceVec,
}

impl Transport {
    pub fn new_test() -> Self {
        Self { sequence: vec![] }
    }
}
