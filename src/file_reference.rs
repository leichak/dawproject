use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FileReference {
    #[serde(rename = "@path")]
    path: String,
    #[serde(rename = "@external")]
    external: Option<bool>,
}
