use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SimpleEffectSchema {
    /// Effect code.
    #[serde(rename = "code")]
    pub code: String,
    /// Effect value.
    #[serde(rename = "value")]
    pub value: i32,
    /// Description of the effect.
    #[serde(rename = "description")]
    pub description: String,
}

impl SimpleEffectSchema {
    pub fn new(code: String, value: i32, description: String) -> SimpleEffectSchema {
        SimpleEffectSchema {
            code,
            value,
            description,
        }
    }
}
