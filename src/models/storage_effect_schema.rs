use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StorageEffectSchema {
    /// Effect code.
    #[serde(rename = "code")]
    pub code: String,
    /// Effect value.
    #[serde(rename = "value")]
    pub value: i32,
}

impl StorageEffectSchema {
    pub fn new(code: String, value: i32) -> StorageEffectSchema {
        StorageEffectSchema { code, value }
    }
}
