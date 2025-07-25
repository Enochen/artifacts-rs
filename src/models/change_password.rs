use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangePassword {
    /// Your password.
    #[serde(rename = "current_password")]
    pub current_password: String,
    /// New password.
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl ChangePassword {
    pub fn new(current_password: String, new_password: String) -> ChangePassword {
        ChangePassword {
            current_password,
            new_password,
        }
    }
}
