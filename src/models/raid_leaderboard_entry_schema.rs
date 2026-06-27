use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RaidLeaderboardEntrySchema {
    /// Leaderboard position.
    #[serde(rename = "position")]
    pub position: u32,
    /// Account name.
    #[serde(rename = "account")]
    pub account: String,
    /// Points earned for this raid instance.
    #[serde(rename = "points")]
    pub points: u32,
}

impl RaidLeaderboardEntrySchema {
    pub fn new(position: u32, account: String, points: u32) -> RaidLeaderboardEntrySchema {
        RaidLeaderboardEntrySchema {
            position,
            account,
            points,
        }
    }
}
