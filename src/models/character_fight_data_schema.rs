use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterFightDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Fight details.
    #[serde(rename = "fight")]
    pub fight: Box<models::FightSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterFightDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        fight: models::FightSchema,
        character: models::CharacterSchema,
    ) -> CharacterFightDataSchema {
        CharacterFightDataSchema {
            cooldown: Box::new(cooldown),
            fight: Box::new(fight),
            character: Box::new(character),
        }
    }
}
