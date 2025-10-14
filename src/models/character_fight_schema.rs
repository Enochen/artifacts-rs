use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterFightSchema {
    /// The result of the fight.
    #[serde(rename = "result")]
    pub result: models::FightResult,
    /// Numbers of the turns of the combat.
    #[serde(rename = "turns")]
    pub turns: i32,
    /// The code of the monster fought.
    #[serde(rename = "opponent")]
    pub opponent: String,
    /// The fight logs.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// Results for each character.
    #[serde(rename = "characters")]
    pub characters: Vec<models::CharacterMultiFightResultSchema>,
}

impl CharacterFightSchema {
    pub fn new(
        result: models::FightResult,
        turns: i32,
        opponent: String,
        logs: Vec<String>,
        characters: Vec<models::CharacterMultiFightResultSchema>,
    ) -> CharacterFightSchema {
        CharacterFightSchema {
            result,
            turns,
            opponent,
            logs,
            characters,
        }
    }
}
