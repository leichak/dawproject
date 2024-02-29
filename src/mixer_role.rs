use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum MixerRoleEnum {
    Regular,
    Master,
    Effect,
    SubMix,
    Vca,
}
