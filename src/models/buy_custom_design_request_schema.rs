use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BuyCustomDesignRequestSchema {
    /// Code of the custom design to purchase.
    #[serde(rename = "code")]
    pub code: String,
}

impl BuyCustomDesignRequestSchema {
    pub fn new(code: String) -> BuyCustomDesignRequestSchema {
        BuyCustomDesignRequestSchema { code }
    }
}
