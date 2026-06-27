use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopSubscriptionResponseDataSchema {
    /// Whether the account is now a member.
    #[serde(rename = "member")]
    pub member: bool,
    /// Membership expiration date.
    #[serde(rename = "member_expiration")]
    pub member_expiration: String,
    /// Remaining gem balance.
    #[serde(rename = "gems")]
    pub gems: i32,
    /// Gem cost of the purchase.
    #[serde(rename = "cost")]
    pub cost: i32,
}

impl GemShopSubscriptionResponseDataSchema {
    pub fn new(
        member: bool,
        member_expiration: String,
        gems: i32,
        cost: i32,
    ) -> GemShopSubscriptionResponseDataSchema {
        GemShopSubscriptionResponseDataSchema {
            member,
            member_expiration,
            gems,
            cost,
        }
    }
}
