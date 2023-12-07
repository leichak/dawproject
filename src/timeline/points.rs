use super::{
    super::unit::Unit, automation_target::AutomationTarget, bool_point::BoolPoint,
    enum_point::EnumPoint, integer_point::IntegerPoint, point::Point, real_point::RealPoint,
    time_signature_point::TimeSignaturePoint, time_unit::TimeUnit,
};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
enum PointsTypeEnum {
    Point(Point),
    RealPoint(RealPoint),
    EnumPoint(EnumPoint),
    BoolPoint(BoolPoint),
    IntegerPoint(IntegerPoint),
    TimeSignaturePoint(TimeSignaturePoint),
}

#[derive(Debug, Deserialize, Serialize)]
enum PointsSequenceEnum {
    Target(AutomationTarget),
    PointType(PointsTypeEnum),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Points {
    // Extends timeline
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@color")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@track")]
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    timeUnit: Option<TimeUnit>,
    // Extension finish
    #[serde(rename = "$value")]
    points: Option<Vec<PointsSequenceEnum>>,
    #[serde(rename = "@unit")]
    unit: Option<Unit>,
}

impl Points {
    pub fn new() -> Self {
        Self {
            id: Some("id" + id_xml.to_string()),
            name: None,
            color: None,
            comment: None,
            track: None,
            timeUnit: None,
            points: None,
            unit: None,
        }
    }
}
