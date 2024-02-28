use crate::application::Application;
use crate::arrangement::Arrangement;

use crate::scene::Scene;

use crate::transport::Transport;

use crate::channel::Channel;
use crate::track::Track;

use serde::Deserialize;
use serde::Serialize;

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
    pub application: Application,
    #[serde(rename = "Transport")]
    transport: Option<Transport>,
    #[serde(rename = "Structure")]
    structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    arrangement: Arrangement,
    #[serde(rename = "Scenes")]
    scenes: Option<Vec<Scene>>,
}

impl Structure {
    pub fn new_empty() -> Self {
        Structure {
            sequence: Vec::new(),
        }
    }
}

impl Project {
    pub fn new() -> Self {
        Project {
            version: "1.0".to_string(),
            application: Application::new_empty(),
            transport: None,
            structure: None,
            arrangement: Arrangement::new_empty(),
            scenes: None,
        }
    }
}
