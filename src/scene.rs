use serde::{Deserialize, Serialize};

use crate::timeline::timeline::TimeLine;

#[derive(Deserialize)]
pub struct Scene {
    pub Timeline: TimeLine,
}
