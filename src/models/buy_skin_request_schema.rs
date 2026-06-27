use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BuySkinRequestSchema {
    /// Code of the skin to purchase.
    #[serde(rename = "code")]
    pub code: String,
}

impl BuySkinRequestSchema {
    pub fn new(code: String) -> BuySkinRequestSchema {
        BuySkinRequestSchema { code }
    }
}
