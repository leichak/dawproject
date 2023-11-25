use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentType {
    audio,
    automation,
    notes,
    video,
    markers,
    tracks,
}
