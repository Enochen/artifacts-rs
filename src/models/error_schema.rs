use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ErrorSchema {
    /// Error code
    #[serde(rename = "code")]
    pub code: i32,
    /// Error message
    #[serde(rename = "message")]
    pub message: String,
    /// Additional error data (used primarily for validation errors)
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ErrorSchema {
    pub fn new(code: i32, message: String) -> ErrorSchema {
        ErrorSchema {
            code,
            message,
            data: None,
        }
    }
}
