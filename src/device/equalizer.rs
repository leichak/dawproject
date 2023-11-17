use serde::Deserialize;

use super::{device::DeviceElements, device_role::DeviceRole, eq_band::EqBand};

use crate::real_parameter::RealParameter;

#[derive(Deserialize, Debug)]
enum EqParamsEnum {
    Band(EqBand),
    InputGain(RealParameter),
    OutputGain(RealParameter),
}

#[derive(Deserialize, Debug)]
struct Equalizer {
    // Extends builtinDevice
    #[serde(rename = "@id")]
    id: String,
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
