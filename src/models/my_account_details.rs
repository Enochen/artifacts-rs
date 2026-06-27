use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MyAccountDetails {
    /// Username.
    #[serde(rename = "username")]
    pub username: String,
    /// Email.
    #[serde(rename = "email")]
    pub email: String,
    /// Member status.
    #[serde(rename = "member")]
    pub member: bool,
    /// Membership expiration date.
    #[serde(rename = "member_expiration", skip_serializing_if = "Option::is_none")]
    pub member_expiration: Option<String>,
    /// Account status.
    #[serde(rename = "status")]
    pub status: models::AccountStatus,
    /// Account badges.
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<String>>,
    /// Skins owned.
    #[serde(rename = "skins")]
    pub skins: Vec<String>,
    /// Gems.
    #[serde(rename = "gems")]
    pub gems: i32,
    /// Member tokens manually granted as rewards for events. Each token can be redeemed for one month of membership.
    #[serde(rename = "member_token", skip_serializing_if = "Option::is_none")]
    pub member_token: Option<i32>,
    /// Achievement points.
    #[serde(rename = "achievements_points")]
    pub achievements_points: i32,
    /// Banned.
    #[serde(rename = "banned")]
    pub banned: bool,
    /// Ban reason.
    #[serde(rename = "ban_reason", skip_serializing_if = "Option::is_none")]
    pub ban_reason: Option<String>,
}

impl MyAccountDetails {
    pub fn new(
        username: String,
        email: String,
        member: bool,
        status: models::AccountStatus,
        skins: Vec<String>,
        gems: i32,
        achievements_points: i32,
        banned: bool,
    ) -> MyAccountDetails {
        MyAccountDetails {
            username,
            email,
            member,
            member_expiration: None,
            status,
            badges: None,
            skins,
            gems,
            member_token: None,
            achievements_points,
            banned,
            ban_reason: None,
        }
    }
}
