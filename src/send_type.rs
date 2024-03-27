use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum SendTypeEnum {
    Pre,
    Post,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SendType {
    #[serde(rename = "$value")]
    field: Vec<SendTypeEnum>,
}
