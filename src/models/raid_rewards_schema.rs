use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RaidRewardsSchema {
    /// Items granted based on cumulative damage dealt.
    #[serde(rename = "damage_rewards", skip_serializing_if = "Option::is_none")]
    pub damage_rewards: Option<Vec<models::RaidDamageRewardSchema>>,
    /// Items granted based on leaderboard rank.
    #[serde(rename = "leaderboard", skip_serializing_if = "Option::is_none")]
    pub leaderboard: Option<Vec<models::RaidRankRewardSchema>>,
}

impl RaidRewardsSchema {
    pub fn new() -> RaidRewardsSchema {
        RaidRewardsSchema {
            damage_rewards: None,
            leaderboard: None,
        }
    }
}
