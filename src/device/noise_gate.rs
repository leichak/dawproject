use serde::Deserialize;
use serde::Serialize;

use super::{device::DeviceElements, device_role::DeviceRole};
use crate::add_one_get;
use crate::real_parameter::RealParameter;

#[derive(Debug, Deserialize, Serialize)]
enum NoiseGateParamsEnum {
    Attack(RealParameter),
    Range(RealParameter),
    Ratio(RealParameter),
    Release(RealParameter),
    Threshold(RealParameter),
}

type NoiseGateParams = Vec<NoiseGateParamsEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct NoiseGate {
    // Extnds builtInDevice
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
    params: NoiseGateParams,
}

impl NoiseGate {
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
