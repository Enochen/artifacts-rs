use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RaidRankRewardSchema {
    /// Inclusive minimum rank.
    #[serde(rename = "min_rank")]
    pub min_rank: u32,
    /// Inclusive maximum rank.
    #[serde(rename = "max_rank")]
    pub max_rank: u32,
    /// Items granted for this rank bracket.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::SimpleItemSchema>>,
}

impl RaidRankRewardSchema {
    pub fn new(min_rank: u32, max_rank: u32) -> RaidRankRewardSchema {
        RaidRankRewardSchema {
            min_rank,
            max_rank,
            items: None,
        }
    }
}
