use crate::models::{CharacterSchema, CooldownSchema};

pub trait GetCooldown {
    fn get_cooldown(&self) -> &CooldownSchema;
}

pub trait GetCharacter {
    fn get_character(&self) -> &CharacterSchema;
}

pub trait IntoData {
    type Data;
    fn into_data(self) -> Self::Data;
}
