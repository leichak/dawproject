use super::device::DeviceElements;
use super::device_role::DeviceRole;
use crate::add_one_get;
use crate::{bool_parameter::BoolParameter, real_parameter::RealParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum CompressorParamsEnum {
    Attack(RealParameter),
    AutoMakeup(BoolParameter),
    InputGain(RealParameter),
    OutputGain(RealParameter),
    Ratio(RealParameter),
    Release(RealParameter),
    Threshold(RealParameter),
}

type CompressorParams = Vec<CompressorParamsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Compressor {
    // Extendes builtInDevice
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "$value")]
    #[serde(default)]
    device_elements: DeviceElements,
    #[serde(rename = "@deviceID")]
    device_id: Option<String>,
    #[serde(rename = "@deviceName")]
    device_name: Option<String>,
    #[serde(rename = "@deviceRole")]
    device_role: Option<DeviceRole>,
    #[serde(rename = "@deviceVendor")]
    device_vendor: Option<String>,
    #[serde(rename = "@loaded")]
    loaded: Option<bool>,
    // Extension ends
    #[serde(default)]
    params: CompressorParams,
}

impl Compressor {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
            params: vec![],
        }
    }
}
