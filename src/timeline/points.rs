use super::timeline::TimeLine;
use super::UpcastTimeline;
use super::{
    super::unit::Unit, automation_target::AutomationTarget, bool_point::BoolPoint,
    enum_point::EnumPoint, integer_point::IntegerPoint, point::Point, real_point::RealPoint,
    time_signature_point::TimeSignaturePoint, time_unit::TimeUnit,
};
use crate::add_one_get;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PointsTypeEnum {
    Point(Point),
    RealPoint(RealPoint),
    EnumPoint(EnumPoint),
    BoolPoint(BoolPoint),
    IntegerPoint(IntegerPoint),
    TimeSignaturePoint(TimeSignaturePoint),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PointsSequenceEnum {
    Target(AutomationTarget),
    PointType(PointsTypeEnum),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Points {
    // Extends timeline
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,
    // Extension finish
    #[serde(rename = "$value")]
    pub points: Option<Vec<PointsSequenceEnum>>,
    #[serde(rename = "@unit")]
    pub unit: Option<Unit>,
}

impl Points {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            points: Some(vec![]),
            unit: None,
        }
    }
}

impl UpcastTimeline for Points {
    // this is to emulate upcasting
    fn upcast(&self) -> TimeLine {
        TimeLine {
            id: self.id.clone(),
            name: self.name.clone(),
            color: self.color.clone(),
            comment: self.comment.clone(),
            track: self.track.clone(),
            time_unit: self.time_unit.clone(),
        }
    }
}
