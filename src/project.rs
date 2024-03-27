use crate::application::Application;
use crate::arrangement::Arrangement;

use crate::mixer_role::MixerRoleEnum;
use crate::scene::Scene;

use crate::transport::Transport;

use crate::channel::Channel;
use crate::track::Track;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Structure {
    #[serde(rename = "$value")]
    pub sequence: Vec<TrackChannelEnum>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    #[serde(rename = "@version")]
    pub version: String, // assign 1.0
    #[serde(rename = "Application")]
    pub application: Application,
    #[serde(rename = "Transport")]
    pub transport: Option<Transport>,
    #[serde(rename = "Structure")]
    pub structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    pub arrangement: Option<Arrangement>,
    #[serde(rename = "Scenes")]
    pub scenes: Option<Vec<Scene>>,
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
            structure: Some(Structure { sequence: vec![] }),
            arrangement: Some(Arrangement::new_test()),
            scenes: None,
        }
    }

    pub fn get_master_track(&self) -> Option<&Track> {
        if let Some(t) = self
            .structure
            .as_ref()
            .unwrap()
            .sequence
            .iter()
            .find(|el| match el {
                TrackChannelEnum::Track(t) => {
                    if t.track_channel
                        .iter()
                        .filter(|el| match el {
                            TrackChannelEnum::Channel(c) => {
                                if c.role.is_some() {
                                    if *c.role.as_ref().unwrap() == MixerRoleEnum::Master {
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        })
                        .count()
                        == 0
                    {
                        false
                    } else {
                        true
                    }
                }
                TrackChannelEnum::Channel(_) => false,
            })
        {
            match t {
                TrackChannelEnum::Track(t) => return Some(&t),
                _ => (),
            }
        }
        None
    }
}
