use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterRestDataSchema {
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// The amount of HP restored.
    #[serde(rename = "hp_restored")]
    pub hp_restored: i32,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterRestDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        hp_restored: i32,
        character: models::CharacterSchema,
    ) -> CharacterRestDataSchema {
        CharacterRestDataSchema {
            cooldown: Box::new(cooldown),
            hp_restored,
            character: Box::new(character),
        }
    }
}
