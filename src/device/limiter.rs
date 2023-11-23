use serde::Deserialize;

use super::{device::DeviceElements, device_role::DeviceRole, eq_band::EqBand};
use crate::real_parameter::RealParameter;

#[derive(Deserialize, Debug)]
enum LimiterParamsEnum {
    Attack(RealParameter),
    InputGain(RealParameter),
    OutputGain(RealParameter),
    Release(RealParameter),
    Threshold(RealParameter),
}

type LimiterParams = Vec<LimiterParamsEnum>;

#[derive(Deserialize, Debug)]
pub struct Limiter {
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
    params: LimiterParams,
}
