use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ContentType {
    audio,
    automation,
    notes,
    video,
    markers,
    tracks,
}
