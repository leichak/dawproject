use serde::Deserialize;

use super::device_role::DeviceRole;
use crate::bool_parameter::BoolParameter;
use crate::file_reference::FileReference;

#[derive(Deserialize, Debug)]
pub enum DeviceElementsEnum {
    Parameters,
    Enabled(BoolParameter),
    State(FileReference),
}

pub type DeviceElements = Vec<DeviceElementsEnum>;

#[derive(Deserialize, Debug)]
enum Parameters {
    parameter,
    RealParameter,
    BoolParameter,
    IntegerParameter,
    EnumParameter,
    TimeSignatureParameter,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    // Extends referenceable
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
}
