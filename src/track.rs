use crate::add_one_get;
use crate::channel::ChannelElementsEnum;
use crate::mixer_role::MixerRoleEnum;
use crate::real_parameter::RealParameter;
use crate::unit::Unit;
use crate::{channel::Channel, content_type::ContentType};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub enum TrackChannelEnum {
    Channel(Channel),
    Track(Track),
}

pub type TrackChannel = Vec<TrackChannelEnum>;

type Content = Vec<ContentType>;

#[derive(Deserialize, Serialize, Debug)]
enum ContentTypeAttribute {
    Content(Content),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Track {
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
    pub track_channel: TrackChannel,
    #[serde(rename = "@contentType")]
    content_type: Content,
    #[serde(rename = "@loaded")]
    loaded: bool,
}

impl Track {
    pub fn new_dummy(
        name: String,
        content_type: Vec<ContentType>,
        mixer_role: Option<MixerRoleEnum>,
        volume: f64,
        pan: f64,
    ) -> Track {
        let volume_parameter = RealParameter::create_empty(volume, Unit::Linear);
        let pan_parameter = RealParameter::create_empty(pan, Unit::Normalized);

        let mut channel = Channel::new_dummy();
        channel.channel_elements = Vec::new();
        channel
            .channel_elements
            .push(ChannelElementsEnum::Pan(pan_parameter));
        channel
            .channel_elements
            .push(ChannelElementsEnum::Volume(volume_parameter));

        channel.role = mixer_role;

        let channel: TrackChannel = vec![TrackChannelEnum::Channel(Channel::new_dummy())];

        Track {
            id: Some(format!("id{}", add_one_get().to_string())),
            name: Some(name),
            color: None,
            comment: None,
            track_channel: channel,
            content_type: content_type,
            loaded: false,
        }
    }

    pub fn new_test(
        name: String,
        content_type: ContentType,
        mixer_role: MixerRoleEnum,
        volume: f64,
        pan: f64,
    ) -> Track {
        Track {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: Some(name),
            color: None,
            comment: None,
            track_channel: vec![],
            content_type: vec![content_type],
            loaded: false,
        }
    }
}
