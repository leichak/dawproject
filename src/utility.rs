use crate::add_one_get;
use crate::content_type::ContentType;
use crate::mixer_role::MixerRoleEnum;
use crate::timeline::{
    audio::Audio, clip::Clip, clips::Clips, time_unit::TimeUnit, timeline::TimeLine, warp::Warp,
};
use crate::track::Track;
use std::fs::File;
use std::io::{prelude::*, ErrorKind};
use std::path::Path;

// create construtctors for all structs
pub fn create_track(
    name: String,
    content_type: Vec<ContentType>,
    mixer_role: MixerRoleEnum,
    volume: f64,
    pan: f64,
) -> Track {
    Track::new_test(name, content_type, mixer_role, volume, pan)
}

pub fn create_audio(
    relative_path: String,
    sample_rate: i32,
    channels: i32,
    duration: f64,
) -> Audio {
    Audio::new_test(relative_path, sample_rate, channels, duration)
}

pub fn create_warp(time: f64, content_time: f64) -> Warp {
    Warp::new_test(time, content_time)
}

pub fn create_clip(content: TimeLine, time: f64, duration: f64) -> Clip {
    // Upcast warps to Timeline
    Clip::new_test(content, time, duration)
}

pub fn create_clips(clips: Vec<Clip>) -> Clips {
    Clips::new_test(clips)
}

pub fn create_file_path_absolute_string(file_name: String) -> Result<String, ErrorKind> {
    let path_str = format!("./{}", file_name);
    let path = Path::new(&path_str);
    let data_file = match File::create(path) {
        Ok(file) => file,
        Err(_) => return Err(ErrorKind::Other),
    };

    match std::fs::canonicalize(path) {
        Ok(p) => return Ok(p.to_str().unwrap().to_string()),
        Err(_) => return Err(ErrorKind::Other),
    }
}
