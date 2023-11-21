use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum WarpSequenceEnum {
    Time(f64),
    ContentTime(f64),
}

type WarpSequence = Vec<WarpSequenceEnum>;

#[derive(Deserialize, Debug)]
pub struct Warp {
    #[serde(rename = "$value")]
    sequence: WarpSequence,
}
