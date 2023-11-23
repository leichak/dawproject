use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub(crate) enum EqBandTypeEnum {
    highPass,
    lowPass,
    bandPass,
    highShelf,
    lowShelf,
    bell,
    notch,
}
