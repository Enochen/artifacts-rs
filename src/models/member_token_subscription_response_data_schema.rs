use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MemberTokenSubscriptionResponseDataSchema {
    /// Whether the account is now a member.
    #[serde(rename = "member")]
    pub member: bool,
    /// Membership expiration date.
    #[serde(rename = "member_expiration")]
    pub member_expiration: String,
    /// Remaining member tokens. Member tokens are manually granted as rewards for events.
    #[serde(rename = "member_token")]
    pub member_token: i32,
}

impl MemberTokenSubscriptionResponseDataSchema {
    pub fn new(
        member: bool,
        member_expiration: String,
        member_token: i32,
    ) -> MemberTokenSubscriptionResponseDataSchema {
        MemberTokenSubscriptionResponseDataSchema {
            member,
            member_expiration,
            member_token,
        }
    }
}
