use crate::add_one_get;
use crate::content_type::ContentType;
use crate::mixer_role::MixerRoleEnum;
use crate::timeline::{
    audio::Audio, clip::Clip, clips::Clips, time_unit::TimeUnit, timeline::TimeLine, warp::Warp,
};
use crate::track::Track;

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
        track_channel: todo!(),
        content_type: vec![content_type],
        loaded: false,
    }
}

pub fn create_audio() -> Audio {
    Audio {
        id: todo!(),
        name: todo!(),
        color: todo!(),
        comment: todo!(),
        track: todo!(),
        timeUnit: todo!(),
        files_sequence: todo!(),
        duration: todo!(),
        algorithm: todo!(),
        channels: todo!(),
        sample_rate: todo!(),
    }
}

pub fn create_warp() -> Warp {
    Warp {
        time: todo!(),
        content_time: todo!(),
    }
}

pub fn create_clip() -> Clip {
    Clip {
        name: todo!(),
        color: todo!(),
        comment: todo!(),
        notes_sequence_choice: todo!(),
        time: todo!(),
        duration: todo!(),
        content_time_unit: todo!(),
        play_start: todo!(),
        play_stop: todo!(),
        loop_start: todo!(),
        loop_end: todo!(),
        fade_time_unit: todo!(),
        fade_in_time: todo!(),
        fade_out_time: todo!(),
        reference: todo!(),
    }
}

pub fn create_clips() -> Clips {
    Clips {
        id: todo!(),
        name: todo!(),
        color: todo!(),
        comment: todo!(),
        track: todo!(),
        time_unit: todo!(),
        clips: todo!(),
    }
}
