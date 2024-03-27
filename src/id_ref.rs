// This contains struct neede to refer to sth with name only
// Not always we want to contains complex type as with Channel Info, but only name of it meaning the type is IDREF

#[derive(Debug, Deserialize, Serialize, Clone)]
struct IdRef {
    id: String,
}
