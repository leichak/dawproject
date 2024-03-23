use crate::add_one_get;
use crate::unit::Unit;
use crate::{
    bool_parameter::BoolParameter, device::au_plugin::AuPlugin,
    device::builtin_device::BuiltinDevice, device::clap_plugin::ClapPlugin,
    device::compressor::Compressor, device::device::Device, device::equalizer::Equalizer,
    device::limiter::Limiter, device::noise_gate::NoiseGate, device::vst2_plugin::Vst2Plugin,
    device::vst3_plugin::Vst3Plugin, mixer_role::MixerRoleEnum, real_parameter::RealParameter,
};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum DeviceTypes {
    Device(Device),
    Vst2Plugin(Vst2Plugin),
    Vst3Plugin(Vst3Plugin),
    ClapPlugin(ClapPlugin),
    BuiltinDevice(BuiltinDevice),
    Equalizer(Equalizer),
    Compressor(Compressor),
    NoiseGate(NoiseGate),
    Limiter(Limiter),
    AuPlugin(AuPlugin),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Devices {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sends {
    #[serde(default)]
    #[serde(rename = "$value")]
    devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChannelElementsEnum {
    Devices(Devices),
    Sends(Sends),
}

type ChannelElements = Vec<ChannelElementsEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    // Extends lane
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@role")]
    pub role: Option<MixerRoleEnum>,
    #[serde(rename = "@audioChannels")]
    audio_channels: i32, // required bc has value default
    #[serde(rename = "@Volume")]
    pub volume: Option<RealParameter>,
    #[serde(rename = "@Pan")]
    pan: Option<RealParameter>,
    #[serde(rename = "@solo")]
    solo: Option<bool>,
    #[serde(rename = "@destination")]
    pub destination: Option<String>, // This is of type id ref ratehr
    #[serde(rename = "$value")] // name should be Devices or Sends and element name accordignly to its name
    #[serde(default)]
    pub channel_elements: ChannelElements,
}

impl Channel {
    pub fn new_test(volume_value: f64, pan_value: f64, role: MixerRoleEnum) -> Self {
        let mut volume = RealParameter::new_required(Unit::Linear);
        volume.value = Some(volume_value);
        let mut pan = RealParameter::new_required(Unit::Normalized);
        pan.value = Some(pan_value);

        Self {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: None,
            color: None,
            comment: None,
            role: Some(role),
            audio_channels: 2,
            volume: Some(volume),
            pan: Some(pan),
            solo: None,
            destination: None,
            channel_elements: vec![],
        }
    }
}
