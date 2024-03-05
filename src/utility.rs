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
    content_type: ContentType,
    mixer_role: MixerRoleEnum,
    volume: f64,
    pan: f64,
) -> Track {
    Track {
        id: Some(format!("id_{}", add_one_get().to_string())),
        name: Some(name),
        color: None,
        comment: None,
        track_channel: vec![],
        content_type: vec![content_type],
        loaded: false,
    }
}

pub fn create_audio(
    relative_path: String,
    sample_rate: i32,
    channels: i32,
    duration: f64,
) -> Audio {
    Audio {
        id: Some(format!("id_{}", add_one_get().to_string())),
        name: None,
        color: None,
        comment: None,
        track: None,
        timeUnit: None,
        files_sequence: None,
        duration: Some(duration),
        algorithm: None,
        channels: Some(channels),
        sample_rate: Some(sample_rate),
    }
}

pub fn create_warp(time: f64, content_time: f64) -> Warp {
    Warp {
        time: time,
        content_time: content_time,
    }
}

pub fn create_clip(content: TimeLine, time: f64, duration: f64) -> Clip {
    Clip {
        name: None,
        color: None,
        comment: None,
        notes_sequence_choice: None,
        time: time,
        duration: Some(duration),
        content_time_unit: None,
        play_start: None,
        play_stop: None,
        loop_start: None,
        loop_end: None,
        fade_time_unit: None,
        fade_in_time: None,
        fade_out_time: None,
        reference: None,
    }
}

pub fn create_clips(clips: Vec<Clip>) -> Clips {
    Clips {
        id: Some(format!("id_{}", add_one_get().to_string())),
        name: None,
        color: None,
        comment: None,
        track: None,
        time_unit: None,
        clips: Some(clips),
    }
}
