use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TokenResponseSchema {
    #[serde(rename = "token")]
    pub token: String,
}

impl TokenResponseSchema {
    pub fn new(token: String) -> TokenResponseSchema {
        TokenResponseSchema { token }
    }
}
