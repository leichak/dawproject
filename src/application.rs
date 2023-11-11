use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Application {
    name: String,
    version: String,
}
