use serde::Deserialize;

#[derive(Deserialize)]
pub enum ContentType {
    audio,
    automation,
    notes,
    video,
    markers,
    tracks,
}
