use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimeUnit {
    #[serde(rename = "@beats")]
    beats,
    #[serde(rename = "@seconds")]
    seconds,
}
