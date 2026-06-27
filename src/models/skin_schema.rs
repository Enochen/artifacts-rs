use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SkinSchema {
    /// The code of the skin. This is the skin's unique identifier.
    #[serde(rename = "code")]
    pub code: String,
    /// Display name of the skin.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the skin and how to obtain it.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether this skin is available to all players by default.
    #[serde(rename = "default")]
    pub default: bool,
    /// Price in gems to purchase this skin. Null if not purchasable.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
}

impl SkinSchema {
    pub fn new(code: String, name: String, description: String, default: bool) -> SkinSchema {
        SkinSchema {
            code,
            name,
            description,
            default,
            price: None,
        }
    }
}
