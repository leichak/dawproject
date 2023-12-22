use super::device::DeviceElements;
use super::device_role::DeviceRole;
use crate::{bool_parameter::BoolParameter, id_xml, real_parameter::RealParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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
    pub fn new() -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id{}", id_xml.to_string())),
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
