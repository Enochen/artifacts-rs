use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnouncementSchema {
    /// Announcement text.
    #[serde(rename = "message")]
    pub message: String,
    /// Datetime of the announcement.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl AnnouncementSchema {
    pub fn new(message: String) -> AnnouncementSchema {
        AnnouncementSchema {
            message,
            created_at: None,
        }
    }
}
