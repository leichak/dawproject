use crate::application::Application;
use crate::arrangement::Arrangement;

use crate::scene::Scene;

use crate::transport::Transport;

use serde::*;

use crate::channel::Channel;
use crate::track::Track;

#[derive(Debug, Deserialize, Serialize)]
enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

#[derive(Debug, Deserialize, Serialize)]
struct Structure {
    #[serde(rename = "$value")]
    sequence: Vec<TrackChannelEnum>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Project {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "Application")]
    application: Application,
    #[serde(rename = "Transport")]
    transport: Transport,
    #[serde(rename = "Structure")]
    structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    arrangement: Arrangement,
    #[serde(rename = "Scenes")]
    scenes: Option<Vec<Scene>>,
}
