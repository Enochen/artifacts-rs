use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddAccountSchema {
    /// Your desired username.
    #[serde(rename = "username")]
    pub username: String,
    /// Your password.
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl AddAccountSchema {
    pub fn new(username: String, password: String) -> AddAccountSchema {
        AddAccountSchema {
            username,
            password,
            email: None,
        }
    }
}
