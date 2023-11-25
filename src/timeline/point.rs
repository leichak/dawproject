use serde::Deserialize;
use serde::Serialize;

// Case when we have 1 type sequence - Then we have just Vec<Type> because point is one type

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    #[serde(rename = "@time")]
    pub time: Option<Vec<String>>,
}
