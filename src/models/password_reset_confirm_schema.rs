use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PasswordResetConfirmSchema {
    /// Password reset token.
    #[serde(rename = "token")]
    pub token: String,
    /// Your new password.
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl PasswordResetConfirmSchema {
    pub fn new(token: String, new_password: String) -> PasswordResetConfirmSchema {
        PasswordResetConfirmSchema {
            token,
            new_password,
        }
    }
}
