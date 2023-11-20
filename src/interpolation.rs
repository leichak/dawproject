use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum InterpolationEnum {
    Hold,
    Linear,
}

#[derive(Deserialize, Debug)]
pub struct Interpolation {
    #[serde(rename = "$value")]
    interpolation_type: Vec<InterpolationEnum>,
}
