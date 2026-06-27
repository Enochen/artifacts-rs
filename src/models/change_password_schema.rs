use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangePasswordSchema {
    /// Your password.
    #[serde(rename = "current_password")]
    pub current_password: String,
    /// New password.
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl ChangePasswordSchema {
    pub fn new(current_password: String, new_password: String) -> ChangePasswordSchema {
        ChangePasswordSchema {
            current_password,
            new_password,
        }
    }
}
