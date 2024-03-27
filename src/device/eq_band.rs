use super::eq_band_type::EqBandTypeEnum;
use crate::{bool_parameter::BoolParameter, real_parameter::RealParameter};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum EqBandParamsEnum {
    Freq(RealParameter),
    Gain(RealParameter),
    Q(RealParameter),
    Enabled(BoolParameter),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EqBand {
    #[serde(default)]
    eq_band_params: Vec<EqBandParamsEnum>,
    #[serde(rename = "@type")]
    eq_type: EqBandTypeEnum,
    #[serde(rename = "@order")]
    order: Option<i32>,
}
