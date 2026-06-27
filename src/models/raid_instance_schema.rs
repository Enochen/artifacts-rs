use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RaidInstanceSchema {
    /// Weekly raid opening datetime in UTC.
    #[serde(rename = "starts_at")]
    pub starts_at: String,
    /// Weekly raid planned closing datetime in UTC.
    #[serde(rename = "ends_at")]
    pub ends_at: String,
    /// Current status of this weekly raid instance.
    #[serde(rename = "status")]
    pub status: models::RaidStatus,
    /// Shared HP pool when this raid instance starts.
    #[serde(rename = "total_hp")]
    pub total_hp: u32,
    /// Remaining shared HP pool for this raid instance.
    #[serde(rename = "remaining_hp")]
    pub remaining_hp: u32,
    /// Number of accounts that contributed during this raid instance.
    #[serde(rename = "participant_count", skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<u32>,
    /// Datetime when this raid instance actually ended.
    #[serde(rename = "ended_at", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Final result for this raid instance.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<models::RaidInstanceResult>,
    /// Datetime when rewards were distributed for this raid instance.
    #[serde(
        rename = "rewards_distributed_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub rewards_distributed_at: Option<String>,
}

impl RaidInstanceSchema {
    pub fn new(
        starts_at: String,
        ends_at: String,
        status: models::RaidStatus,
        total_hp: u32,
        remaining_hp: u32,
    ) -> RaidInstanceSchema {
        RaidInstanceSchema {
            starts_at,
            ends_at,
            status,
            total_hp,
            remaining_hp,
            participant_count: None,
            ended_at: None,
            result: None,
            rewards_distributed_at: None,
        }
    }
}
