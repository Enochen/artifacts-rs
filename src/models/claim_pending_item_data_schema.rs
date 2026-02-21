use crate::models;
use serde::{Deserialize, Serialize};

/// ClaimPendingItemDataSchema : Data schema for claim pending item action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClaimPendingItemDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// The claimed pending item.
    #[serde(rename = "item")]
    pub item: Box<models::PendingItemSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl ClaimPendingItemDataSchema {
    /// Data schema for claim pending item action.
    pub fn new(
        cooldown: models::CooldownSchema,
        item: models::PendingItemSchema,
        character: models::CharacterSchema,
    ) -> ClaimPendingItemDataSchema {
        ClaimPendingItemDataSchema {
            cooldown: Box::new(cooldown),
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for ClaimPendingItemDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for ClaimPendingItemDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
