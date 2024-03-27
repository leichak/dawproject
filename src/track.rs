use crate::add_one_get;
use crate::channel::ChannelElementsEnum;
use crate::mixer_role::MixerRoleEnum;
use crate::project::TrackChannelEnum;
use crate::real_parameter::RealParameter;
use crate::unit::Unit;
use crate::{channel::Channel, content_type::ContentType};
use serde::Deserialize;
use serde::Serialize;

pub type TrackChannel = Vec<TrackChannelEnum>;

type Content = Vec<ContentType>;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum ContentTypeAttribute {
    Content(Content),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    // Extends lane
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
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
    pub fn new_test(
        name: String,
        content_type: Vec<ContentType>,
        mixer_role: MixerRoleEnum,
        volume: f64,
        pan: f64,
    ) -> Track {
        let channel = Channel::new_test(volume, pan, mixer_role);

        Track {
            id: Some(format!("id_{}", add_one_get().to_string())),
            name: Some(name),
            color: None,
            comment: None,
            track_channel: vec![TrackChannelEnum::Channel(channel)],
            content_type: content_type,
            loaded: false,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone().unwrap()
    }
}
