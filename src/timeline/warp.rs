use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum WarpSequenceEnum {
    Time(f64),
    ContentTime(f64),
}

type WarpSequence = Vec<WarpSequenceEnum>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Warp {
    // #[serde(rename = "$value")]
    // sequence: Option<WarpSequence>,
    #[serde(rename = "@time")]
    time: f64,
    #[serde(rename = "@contentTime")]
    content_time: f64,
}
