use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum EqBandTypeEnum {
    highPass,
    lowPass,
    bandPass,
    highShelf,
    lowShelf,
    bell,
    notch,
}
