#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum SendTypeEnum {
    Pre,
    Post,
}

#[derive(Deserialize)]
struct SendType {
    #[serde(rename = "$value")]
    field: Vec<SendTypeEnum>,
}
