use self::timeline::TimeLine;

pub mod audio;
pub mod automation_target;
pub mod bool_point;
pub mod clip;
pub mod clip_slot;
pub mod clips;
pub mod enum_point;
pub mod integer_point;
pub mod lanes;
pub mod marker;
pub mod markers;
pub mod media_file;
pub mod note;
pub mod notes;
pub mod point;
pub mod points;
pub mod real_point;
pub mod time_signature_point;
pub mod time_unit;
pub mod timeline;
pub mod video;
pub mod warp;
pub mod warps;

// This trait is for imitating upcast behaviour when needed, like in Clip that holds Timeline that can be Warps type
pub trait UpcastTimeline {
    fn upcast(&self) -> TimeLine;
}
