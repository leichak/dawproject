use serde::Deserialize;
use serde::Serialize;

use super::{device::DeviceElements, device_role::DeviceRole, eq_band::EqBand};
use crate::add_one_get;
use crate::real_parameter::RealParameter;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum EqParamsEnum {
    Band(EqBand),
    InputGain(RealParameter),
    OutputGain(RealParameter),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Equalizer {
    // Extends builtinDevice
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
    // End of extension
    #[serde(default)]
    eq_band_params: Vec<EqParamsEnum>,
}

impl Equalizer {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
            eq_band_params: vec![],
        }
    }
}
