use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FightSchema {
    /// The amount of xp gained from the fight.
    #[serde(rename = "xp")]
    pub xp: i32,
    /// The amount of gold gained from the fight.
    #[serde(rename = "gold")]
    pub gold: i32,
    /// The items dropped from the fight.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropSchema>,
    /// Numbers of the turns of the combat.
    #[serde(rename = "turns")]
    pub turns: i32,
    /// The fight logs.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// The result of the fight.
    #[serde(rename = "result")]
    pub result: models::FightResult,
}

impl FightSchema {
    pub fn new(
        xp: i32,
        gold: i32,
        drops: Vec<models::DropSchema>,
        turns: i32,
        logs: Vec<String>,
        result: models::FightResult,
    ) -> FightSchema {
        FightSchema {
            xp,
            gold,
            drops,
            turns,
            logs,
            result,
        }
    }
}
