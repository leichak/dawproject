use crate::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Transport {
    pub tempo: RealParameter,
    pub time_signature: TimeSignatureParameter,
}
