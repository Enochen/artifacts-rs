use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CombatResultSchema {
    /// Combat result: 'win' or 'loss'.
    #[serde(rename = "result")]
    pub result: String,
    /// Number of turns the combat lasted.
    #[serde(rename = "turns")]
    pub turns: i32,
    /// Combat logs.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// Character results from combat.
    #[serde(rename = "character_results")]
    pub character_results: Vec<serde_json::Value>,
}

impl CombatResultSchema {
    pub fn new(
        result: String,
        turns: i32,
        logs: Vec<String>,
        character_results: Vec<serde_json::Value>,
    ) -> CombatResultSchema {
        CombatResultSchema {
            result,
            turns,
            logs,
            character_results,
        }
    }
}
