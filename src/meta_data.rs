use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

struct MetaData {
    meta_data: MetaDataVec,
}
