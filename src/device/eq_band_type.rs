use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum EqBandTypeEnum {
    highPass,
    lowPass,
    bandPass,
    highShelf,
    lowShelf,
    bell,
    notch,
}
