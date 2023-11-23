use crate::time_signature_parameter::TimeSignatureParameter;
use crate::track::Track;

use super::{
    super::unit::Unit, automation_target::AutomationTarget, bool_point::BoolPoint,
    enum_point::EnumPoint, integer_point::IntegerPoint, point::Point, real_point::RealPoint,
    time_signature_point::TimeSignaturePoint, time_unit::TimeUnit,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
enum PointsTypeEnum {
    Point(Point),
    RealPoint(RealPoint),
    EnumPoint(EnumPoint),
    BoolPoint(BoolPoint),
    IntegerPoint(IntegerPoint),
    TimeSignaturePoint(TimeSignaturePoint),
}

#[derive(Deserialize, Debug)]
enum PointsSequenceEnum {
    Target(AutomationTarget),
    PointType(PointsTypeEnum),
}

#[derive(Deserialize, Debug)]
pub struct Points {
    // Extends timeline
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    comment: Option<String>, // att
    #[serde(rename = "@track")]
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension finish
    #[serde(rename = "$value")]
    points: Option<Vec<PointsSequenceEnum>>, //The contained points. They should all be of the same type and match the target parameter. */
    #[serde(rename = "@unit")]
    unit: Option<Unit>,
}
