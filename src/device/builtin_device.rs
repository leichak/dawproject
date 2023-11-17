use super::device::DeviceElements;
use super::device_role::DeviceRole;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct BuiltinDevice {
    // Extends device
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
}
