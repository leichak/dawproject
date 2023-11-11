use serde::{Deserialize, Serialize};

/* This is struct that needs to provide unique id, in original code it derives from referencable */

#[derive(Deserialize)]
pub struct Lane {
    #[serde(rename = "@id")]
    id: String,
}

impl Lane {
    pub fn new() -> Self {
        Lane {
            id: "random_string".to_string(),
        }
    }
}
