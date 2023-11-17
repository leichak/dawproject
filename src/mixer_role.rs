use serde::Deserialize;

#[derive(Deserialize)]
enum MixerRole {
    regular,
    master,
    effecttrack,
    submix,
    vca,
}
