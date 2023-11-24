use crate::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
enum TransportSequence {
    Tempo(RealParameter),
    TimeSignature(TimeSignatureParameter),
}

type TransportSequenceVec = Vec<TransportSequence>;

#[derive(Deserialize, Debug)]
pub struct Transport {
    #[serde(rename = "$value")]
    sequence: TransportSequenceVec, // #[serde(rename = "Tempo")]
                                    // pub tempo: RealParameter,
                                    // #[serde(rename = "TimeSignature")]
                                    // pub time_signature: TimeSignatureParameter,
}
