use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ResponseSchema {
    #[serde(rename = "message")]
    pub message: String,
}

impl ResponseSchema {
    pub fn new(message: String) -> ResponseSchema {
        ResponseSchema { message }
    }
}
