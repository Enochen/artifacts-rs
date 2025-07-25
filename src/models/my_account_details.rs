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
    #[serde(
        rename = "member_expiration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_expiration: Option<Option<String>>,
    /// Account status.
    #[serde(rename = "status")]
    pub status: models::AccountStatus,
    /// Account badges.
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<serde_json::Value>>,
    /// Skins owned.
    #[serde(rename = "skins")]
    pub skins: Vec<serde_json::Value>,
    /// Gems.
    #[serde(rename = "gems")]
    pub gems: i32,
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
        skins: Vec<serde_json::Value>,
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
            achievements_points,
            banned,
            ban_reason: None,
        }
    }
}
