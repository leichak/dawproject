use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum TimeUnit {
    beats,
    seconds,
}
