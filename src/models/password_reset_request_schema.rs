use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordResetRequestSchema {
    /// Your email address.
    #[serde(rename = "email")]
    pub email: String,
}

impl PasswordResetRequestSchema {
    pub fn new(email: String) -> PasswordResetRequestSchema {
        PasswordResetRequestSchema { email }
    }
}
