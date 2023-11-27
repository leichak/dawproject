use serde::Deserialize;
use serde::Serialize;

use crate::{
    bool_parameter::BoolParameter, device::au_plugin::AuPlugin,
    device::builtin_device::BuiltinDevice, device::clap_plugin::ClapPlugin,
    device::compressor::Compressor, device::device::Device, device::equalizer::Equalizer,
    device::limiter::Limiter, device::noise_gate::NoiseGate, device::vst2_plugin::Vst2Plugin,
    device::vst3_plugin::Vst3Plugin, mixer_role::MixerRoleEnum, real_parameter::RealParameter,
};

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
struct Devices {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sends {
    #[serde(default)]
    #[serde(rename = "$value")]
    devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChannelElementsEnum {
    Devices(Devices),
    Pan(RealParameter),
    Mute(BoolParameter),
    Volume(RealParameter),
    Sends(Sends),
}

type ChannelElements = Vec<ChannelElementsEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Channel {
    // Extends lane
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "$value")]
    #[serde(default)]
    pub channel_elements: ChannelElements,
    #[serde(rename = "@audioChannels")]
    audio_channels: Option<i32>,
    #[serde(rename = "@destination")]
    destination: Option<String>,
    #[serde(rename = "@role")]
    pub role: Option<MixerRoleEnum>,
    #[serde(rename = "@solo")]
    solo: Option<bool>,
}

impl Channel {
    pub fn new_dummy() -> Self {
        Channel {
            id: None,
            name: None,
            color: None,
            comment: None,
            channel_elements: Vec::new(),
            audio_channels: None,
            destination: None,
            role: None,
            solo: None,
        }
    }
}
