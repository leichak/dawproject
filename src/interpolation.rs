use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationEnum {
    Hold,
    Linear,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interpolation {
    #[serde(rename = "$value")]
    interpolation_type: Vec<InterpolationEnum>,
}

impl Interpolation {
    pub fn create(interpolation_type: InterpolationEnum) -> Self {
        Self {
            interpolation_type: vec![interpolation_type],
        }
    }
}
