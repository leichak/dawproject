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
enum DeviceTypes {
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
    devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sends {
    #[serde(default)]
    #[serde(rename = "$value")]
    devices: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
enum ChannelElementsEnum {
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
    channel_elements: ChannelElements,
    #[serde(rename = "@audioChannels")]
    audio_channels: i32,
    #[serde(rename = "@destination")]
    destination: Option<String>,
    #[serde(rename = "@role")]
    role: Option<MixerRoleEnum>,
    #[serde(rename = "@solo")]
    solo: Option<bool>,
}
