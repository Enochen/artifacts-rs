use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePassword {
    /// Your password.
    #[serde(rename = "password")]
    pub password: String,
}

impl ChangePassword {
    pub fn new(password: String) -> ChangePassword {
        ChangePassword { password }
    }
}
