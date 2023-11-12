use crate::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Transport {
    #[serde(rename = "Tempo")]
    pub tempo: RealParameter,
    #[serde(rename = "TimeSignature")]
    pub time_signature: TimeSignatureParameter,
}
