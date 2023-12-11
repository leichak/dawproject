use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum SendTypeEnum {
    Pre,
    Post,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SendType {
    #[serde(rename = "$value")]
    field: Vec<SendTypeEnum>,
}
