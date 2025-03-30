use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleEffectSchema {
    /// Effect code.
    #[serde(rename = "code")]
    pub code: String,
    /// Effect value.
    #[serde(rename = "value")]
    pub value: i32,
}

impl SimpleEffectSchema {
    pub fn new(code: String, value: i32) -> SimpleEffectSchema {
        SimpleEffectSchema { code, value }
    }
}
