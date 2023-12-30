use crate::id_xml;

use super::device::DeviceElements;
use super::device_role::DeviceRole;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClapPlugin {
    // Extends Plugin
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

impl ClapPlugin {
    pub fn new_empty()  -> Self {
        id_xml += 1;
        Self {
            id: Some(format!("id{}", id_xml.to_string())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
            plugin_version: None,
        }
    }
}
