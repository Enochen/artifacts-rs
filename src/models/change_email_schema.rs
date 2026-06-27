use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangeEmailSchema {
    /// Your current email.
    #[serde(rename = "current_email")]
    pub current_email: String,
    /// New email.
    #[serde(rename = "new_email")]
    pub new_email: String,
}

impl ChangeEmailSchema {
    pub fn new(current_email: String, new_email: String) -> ChangeEmailSchema {
        ChangeEmailSchema {
            current_email,
            new_email,
        }
    }
}
