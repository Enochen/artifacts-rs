use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct FightRequestSchema {
    /// Optional list of additional character names to include in the fight (max 2 additional characters).
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
}

impl FightRequestSchema {
    pub fn new() -> FightRequestSchema {
        FightRequestSchema { participants: None }
    }
}
