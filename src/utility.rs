use crate::add_one_get;
use crate::content_type::ContentType;
use crate::mixer_role::MixerRoleEnum;
use crate::timeline::{
    audio::Audio, clip::Clip, clips::Clips, time_unit::TimeUnit, timeline::TimeLine, warp::Warp,
};
use crate::track::Track;

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
    Clip::new_test(content, time, duration)
}

pub fn create_clips(clips: Vec<Clip>) -> Clips {
    Clips::new_test(clips)
}
