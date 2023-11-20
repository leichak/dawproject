use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum TimeUnit {
    #[serde(rename = "@beats")]
    beats,
    #[serde(rename = "@seconds")]
    seconds,
}
