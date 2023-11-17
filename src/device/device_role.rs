use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub(crate) enum DeviceRole {
    instrument,
    noteFX,
    audioFX,
    analyzer,
}
