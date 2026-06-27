use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemTransactionSchema {
    /// Gem transaction type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Signed gem delta.
    #[serde(rename = "gems")]
    pub gems: i32,
    /// Human-readable transaction description.
    #[serde(rename = "description")]
    pub description: String,
    /// Additional transaction metadata.
    #[serde(rename = "metadata")]
    #[cfg_attr(feature = "specta", specta(type = specta_util::Unknown))]
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    /// Transaction creation date.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl GemTransactionSchema {
    pub fn new(
        r#type: String,
        gems: i32,
        description: String,
        metadata: std::collections::HashMap<String, serde_json::Value>,
        created_at: String,
    ) -> GemTransactionSchema {
        GemTransactionSchema {
            r#type,
            gems,
            description,
            metadata,
            created_at,
        }
    }
}
