use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum WarpSequenceEnum {
    Time(f64),
    ContentTime(f64),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Warp {
    #[serde(rename = "@time")]
    time: f64,
    #[serde(rename = "@contentTime")]
    content_time: f64,
}

impl Warp {
    pub fn new_test(time: f64, content_time: f64) -> Self {
        Self { time, content_time }
    }
}
