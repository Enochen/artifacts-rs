use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EffectSchema {
    /// Name of the effect.
    #[serde(rename = "name")]
    pub name: String,
    /// The code of the effect. This is the effect's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Description of the effect. This is a brief description of the effect.
    #[serde(rename = "description")]
    pub description: String,
    /// Type of the effect.
    #[serde(rename = "type")]
    pub r#type: models::EffectType,
    /// Subtype of the effect.
    #[serde(rename = "subtype")]
    pub subtype: models::EffectSubtype,
}

impl EffectSchema {
    pub fn new(
        name: String,
        code: String,
        description: String,
        r#type: models::EffectType,
        subtype: models::EffectSubtype,
    ) -> EffectSchema {
        EffectSchema {
            name,
            code,
            description,
            r#type,
            subtype,
        }
    }
}
