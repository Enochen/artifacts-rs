use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillInfoSchema {
    /// The amount of xp gained.
    #[serde(rename = "xp")]
    pub xp: i32,
    /// Objects received.
    #[serde(rename = "items")]
    pub items: Vec<models::DropSchema>,
}

impl SkillInfoSchema {
    pub fn new(xp: i32, items: Vec<models::DropSchema>) -> SkillInfoSchema {
        SkillInfoSchema { xp, items }
    }
}
