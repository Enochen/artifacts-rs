use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AccountDetails {
    /// Username.
    #[serde(rename = "username")]
    pub username: String,
    /// Member status.
    #[serde(rename = "member")]
    pub member: bool,
    /// Account status.
    #[serde(rename = "status")]
    pub status: models::AccountStatus,
    /// Account badges.
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<Vec<specta_util::Unknown>>))]
    pub badges: Option<Vec<serde_json::Value>>,
    /// Skins owned.
    #[serde(rename = "skins")]
    #[cfg_attr(feature = "specta", specta(type = Vec<specta_util::Unknown>))]
    pub skins: Vec<serde_json::Value>,
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

impl AccountDetails {
    pub fn new(
        username: String,
        member: bool,
        status: models::AccountStatus,
        skins: Vec<serde_json::Value>,
        achievements_points: i32,
        banned: bool,
    ) -> AccountDetails {
        AccountDetails {
            username,
            member,
            status,
            badges: None,
            skins,
            achievements_points,
            banned,
            ban_reason: None,
        }
    }
}
