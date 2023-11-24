use crate::{channel::Channel, content_type::ContentType};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
enum TrackChannelEnum {
    Channel(Channel),
    Track(Track),
}

type TrackChannel = Vec<TrackChannelEnum>;

type Content = Vec<ContentType>;

#[derive(Deserialize, Debug)]
enum ContentTypeAttribute {
    Content(Content),
}

#[derive(Deserialize, Debug)]
pub(crate) struct Track {
    // Extends lane
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "$value")]
    track_channel: TrackChannel,
    #[serde(rename = "@contentType")]
    content_type: Content,
    #[serde(rename = "@loaded")]
    loaded: bool,
}
