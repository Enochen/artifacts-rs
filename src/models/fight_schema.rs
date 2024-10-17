use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FightSchema {
    /// The amount of xp gained by the fight.
    #[serde(rename = "xp")]
    pub xp: i32,
    /// The amount of gold gained by the fight.
    #[serde(rename = "gold")]
    pub gold: i32,
    /// The items dropped by the fight.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropSchema>,
    /// Numbers of the turns of the combat.
    #[serde(rename = "turns")]
    pub turns: i32,
    /// The amount of blocked hits by the monster.
    #[serde(rename = "monster_blocked_hits")]
    pub monster_blocked_hits: Box<models::BlockedHitsSchema>,
    /// The amount of blocked hits by the player.
    #[serde(rename = "player_blocked_hits")]
    pub player_blocked_hits: Box<models::BlockedHitsSchema>,
    /// The fight logs.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// The result of the fight.
    #[serde(rename = "result")]
    pub result: Result,
}

impl FightSchema {
    pub fn new(
        xp: i32,
        gold: i32,
        drops: Vec<models::DropSchema>,
        turns: i32,
        monster_blocked_hits: models::BlockedHitsSchema,
        player_blocked_hits: models::BlockedHitsSchema,
        logs: Vec<String>,
        result: Result,
    ) -> FightSchema {
        FightSchema {
            xp,
            gold,
            drops,
            turns,
            monster_blocked_hits: Box::new(monster_blocked_hits),
            player_blocked_hits: Box::new(player_blocked_hits),
            logs,
            result,
        }
    }
}
/// The result of the fight.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "lose")]
    Lose,
}

impl Default for Result {
    fn default() -> Result {
        Self::Win
    }
}
