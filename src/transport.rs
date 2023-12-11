use crate::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
enum TransportSequence {
    Tempo(RealParameter),
    TimeSignature(TimeSignatureParameter),
}

type TransportSequenceVec = Vec<TransportSequence>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transport {
    #[serde(rename = "$value")]
    sequence: TransportSequenceVec,
}
