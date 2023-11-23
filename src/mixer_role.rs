use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) enum MixerRoleEnum {
    Regular,
    Master,
    Effect,
    SubMix,
    Vca,
}
