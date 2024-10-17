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
    /// Your email.
    #[serde(rename = "email")]
    pub email: String,
}

impl AddAccountSchema {
    pub fn new(username: String, password: String, email: String) -> AddAccountSchema {
        AddAccountSchema {
            username,
            password,
            email,
        }
    }
}
