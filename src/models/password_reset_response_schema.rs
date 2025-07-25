use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PasswordResetResponseSchema {
    /// Success message.
    #[serde(rename = "message")]
    pub message: String,
}

impl PasswordResetResponseSchema {
    pub fn new(message: String) -> PasswordResetResponseSchema {
        PasswordResetResponseSchema { message }
    }
}
