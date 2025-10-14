use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ActiveCharacterSchema {
    /// Name of the character.
    #[serde(rename = "name")]
    pub name: String,
    /// Account name.
    #[serde(rename = "account")]
    pub account: String,
    /// Character skin code.
    #[serde(rename = "skin")]
    pub skin: models::CharacterSkin,
    /// Character x coordinate.
    #[serde(rename = "x")]
    pub x: i32,
    /// Character y coordinate.
    #[serde(rename = "y")]
    pub y: i32,
    /// Character current layer.
    #[serde(rename = "layer")]
    pub layer: models::MapLayer,
    /// Character current map ID.
    #[serde(rename = "map_id")]
    pub map_id: i32,
}

impl ActiveCharacterSchema {
    pub fn new(
        name: String,
        account: String,
        skin: models::CharacterSkin,
        x: i32,
        y: i32,
        layer: models::MapLayer,
        map_id: i32,
    ) -> ActiveCharacterSchema {
        ActiveCharacterSchema {
            name,
            account,
            skin,
            x,
            y,
            layer,
            map_id,
        }
    }
}
