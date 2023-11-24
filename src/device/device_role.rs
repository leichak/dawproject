use serde::Deserialize;
#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum DeviceRole {
    instrument,
    noteFX,
    audioFX,
    analyzer,
}
