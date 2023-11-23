use super::{device::DeviceElements, device_role::DeviceRole};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vst2Plugin {
    // Extends plugin
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
    #[serde(rename = "@pluginVersion")]
    plugin_version: Option<String>,
}
