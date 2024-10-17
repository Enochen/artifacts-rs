use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemSchema {
    /// Item name.
    #[serde(rename = "name")]
    pub name: String,
    /// Item code. This is the item's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Item level.
    #[serde(rename = "level")]
    pub level: u32,
    /// Item type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Item subtype.
    #[serde(rename = "subtype")]
    pub subtype: String,
    /// Item description.
    #[serde(rename = "description")]
    pub description: String,
    /// List of object effects. For equipment, it will include item stats.
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<models::ItemEffectSchema>>,
    #[serde(
        rename = "craft",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub craft: Option<Option<Box<models::CraftSchema>>>,
}

impl ItemSchema {
    pub fn new(
        name: String,
        code: String,
        level: u32,
        r#type: String,
        subtype: String,
        description: String,
    ) -> ItemSchema {
        ItemSchema {
            name,
            code,
            level,
            r#type,
            subtype,
            description,
            effects: None,
            craft: None,
        }
    }
}
