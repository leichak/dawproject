use crate::mixer_role::MixerRoleEnum;
use crate::real_parameter::RealParameter;
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
pub(crate) struct Track {
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
    track_channel: TrackChannel,
    #[serde(rename = "@contentType")]
    content_type: Content,
    #[serde(rename = "@loaded")]
    loaded: bool,
}

impl Track {
    pub fn create_track(
        name: String,
        content_type: Vec<ContentType>,
        mixer_role: MixerRoleEnum,
        volume: f64,
        pan: f64,
    ) -> Track {
        let channel: TrackChannel = vec![TrackChannelEnum::Channel(Channel::new_empty())];
        let volume_parameter = RealParameter{
            id: todo!(),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
            parameter_id: todo!(),
            value: todo!(),
            unit: todo!(),
            min: todo!(),
            max: todo!(),
        }

        Track {
            id: None,
            name: Some(name),
            color: None,
            comment: None,
            track_channel: channel,
            content_type: Vec::new(),
            loaded: false,
        }
    }
}
