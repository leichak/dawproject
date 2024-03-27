use serde::{Deserialize, Serialize};

use super::device_role::DeviceRole;
use crate::add_one_get;
use crate::bool_parameter::BoolParameter;
use crate::file_reference::FileReference;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeviceElementsEnum {
    Parameters,
    Enabled(BoolParameter),
    State(FileReference),
}

pub type DeviceElements = Vec<DeviceElementsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Parameters {
    parameter,
    RealParameter,
    BoolParameter,
    IntegerParameter,
    EnumParameter,
    TimeSignatureParameter,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl Device {
    pub fn new_empty() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get().to_string())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
        }
    }
}
