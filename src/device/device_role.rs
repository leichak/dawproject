use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeviceRole {
    instrument,
    noteFX,
    audioFX,
    analyzer,
}
