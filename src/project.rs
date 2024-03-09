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
pub struct Project {
    #[serde(rename = "@version")]
    pub version: String, // assign 1.0
    #[serde(rename = "Application")]
    pub application: Application,
    #[serde(rename = "Transport")]
    transport: Option<Transport>,
    #[serde(rename = "Structure")]
    structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    arrangement: Option<Arrangement>,
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
    pub fn new_test(name: String, version: f64) -> Self {
        Project {
            version: "1.0".to_string(),
            application: Application::new_name_ver(name, version),
            transport: None,
            structure: None,
            arrangement: Arrangement::new_test(),
            scenes: None,
        }
    }
}
