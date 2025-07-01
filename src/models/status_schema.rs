use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusSchema {
    /// Game version.
    #[serde(rename = "version")]
    pub version: String,
    /// Server time.
    #[serde(rename = "server_time")]
    pub server_time: String,
    /// Maximum level.
    #[serde(rename = "max_level")]
    pub max_level: i32,
    /// Maximum skill level.
    #[serde(rename = "max_skill_level")]
    pub max_skill_level: i32,
    /// Characters online.
    #[serde(rename = "characters_online")]
    pub characters_online: i32,
    /// Current season details.
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<Box<models::SeasonSchema>>,
    /// Server announcements.
    #[serde(rename = "announcements")]
    pub announcements: Vec<models::AnnouncementSchema>,
    /// Rate limits.
    #[serde(rename = "rate_limits")]
    pub rate_limits: Vec<models::RateLimitSchema>,
}

impl StatusSchema {
    pub fn new(
        version: String,
        server_time: String,
        max_level: i32,
        max_skill_level: i32,
        characters_online: i32,
        announcements: Vec<models::AnnouncementSchema>,
        rate_limits: Vec<models::RateLimitSchema>,
    ) -> StatusSchema {
        StatusSchema {
            version,
            server_time,
            max_level,
            max_skill_level,
            characters_online,
            season: None,
            announcements,
            rate_limits,
        }
    }
}
