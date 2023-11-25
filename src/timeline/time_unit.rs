use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum TimeUnit {
    beats,
    seconds,
}
