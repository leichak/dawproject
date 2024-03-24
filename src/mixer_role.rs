use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum MixerRoleEnum {
    Regular,
    Master,
    Effect,
    SubMix,
    Vca,
}
