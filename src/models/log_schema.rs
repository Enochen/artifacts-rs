use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LogSchema {
    /// Character name.
    #[serde(rename = "character")]
    pub character: String,
    /// Account character.
    #[serde(rename = "account")]
    pub account: String,
    /// Type of action.
    #[serde(rename = "type")]
    pub r#type: models::LogType,
    /// Description of action.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub content: Option<serde_json::Value>,
    /// Cooldown in seconds.
    #[serde(rename = "cooldown")]
    pub cooldown: i32,
    #[serde(
        rename = "cooldown_expiration",
        deserialize_with = "Option::deserialize"
    )]
    pub cooldown_expiration: Option<String>,
    /// Datetime of creation.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl LogSchema {
    pub fn new(
        character: String,
        account: String,
        r#type: models::LogType,
        description: String,
        content: Option<serde_json::Value>,
        cooldown: i32,
        cooldown_expiration: Option<String>,
        created_at: String,
    ) -> LogSchema {
        LogSchema {
            character,
            account,
            r#type,
            description,
            content,
            cooldown,
            cooldown_expiration,
            created_at,
        }
    }
}
