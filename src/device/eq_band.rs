use super::eq_band_type::EqBandTypeEnum;
use crate::{bool_parameter::BoolParameter, real_parameter::RealParameter};
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
enum EqBandParamsEnum {
    Freq(RealParameter),
    Gain(RealParameter),
    Q(RealParameter),
    Enabled(BoolParameter),
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EqBand {
    #[serde(default)]
    eq_band_params: Vec<EqBandParamsEnum>,
    #[serde(rename = "@type")]
    eq_type: EqBandTypeEnum,
    #[serde(rename = "@order")]
    order: Option<i32>,
}
