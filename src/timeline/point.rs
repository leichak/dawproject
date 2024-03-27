use serde::Deserialize;
use serde::Serialize;

// Case when we have 1 type sequence - Then we have just Vec<Type> because point is one type

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Point {
    #[serde(rename = "@time")]
    pub time: Option<f64>,
}

impl Point {
    pub fn new_empty() -> Self {
        Self { time: None }
    }
}
