use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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
    /// Item conditions. If applicable. Conditions for using or equipping the item.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::ConditionSchema>>,
    /// List of object effects. For equipment, it will include item stats.
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<models::SimpleEffectSchema>>,
    #[serde(
        rename = "craft",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub craft: Option<Option<Box<models::CraftSchema>>>,
    /// Item tradeable status. A non-tradeable item cannot be exchanged or sold.
    #[serde(rename = "tradeable")]
    pub tradeable: bool,
}

impl ItemSchema {
    pub fn new(
        name: String,
        code: String,
        level: u32,
        r#type: String,
        subtype: String,
        description: String,
        tradeable: bool,
    ) -> ItemSchema {
        ItemSchema {
            name,
            code,
            level,
            r#type,
            subtype,
            description,
            conditions: None,
            effects: None,
            craft: None,
            tradeable,
        }
    }
}
