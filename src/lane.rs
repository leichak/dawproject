use serde::{Deserialize, Serialize};

/* This is struct that needs to provide unique id, in original code it derives from referencable */

#[derive(Deserialize)]
pub struct Lane {
    id: String,
}

impl Lane {
    pub fn new() -> Self {
        // It should return string id unique
        Lane {
            id: "random_string".to_string(),
        }
    }
}
