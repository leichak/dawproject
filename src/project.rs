/*
This struct represents project, it conists following data:
- version of DawProject format his file was saved on
- metadata (name/version) about the application that saved this file
- transport element containing playback parameters such as Tempo and Time-signatures
- track/channel structure of this file (list of Lane)
- arrangement timeline of this file
- Clip Launcher scenes of this file
 */

use crate::application::Application;
use crate::arrangement::Arrangement;

use crate::scene::Scene;

use crate::transport::Transport;

use serde::*;

// Here name of field corresponds to xml

use crate::channel::Channel;
use crate::track::Track;

#[derive(Deserialize, Debug)]
enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

#[derive(Deserialize, Debug)]
struct Structure {
    #[serde(rename = "$value")]
    sequence: Vec<TrackChannelEnum>,
}

#[derive(serde::Deserialize, Debug)]
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
