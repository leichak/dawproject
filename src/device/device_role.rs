use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum DeviceRole {
    instrument,
    noteFX,
    audioFX,
    analyzer,
}
