use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum MetaDataEnum {
    Title(String),
    Artist(String),
    Album(String),
    OriginalArtist(String),
    Composer(String),
    Songwriter(String),
    Producer(String),
    Arranger(String),
    Year(String),
    Genre(String),
    Copyright(String),
    Website(String),
    Comment(String),
}

type MetaDataVec = Vec<MetaDataEnum>;

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    meta_data: MetaDataVec,
}
