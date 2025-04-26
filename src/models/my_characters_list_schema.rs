use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyCharactersListSchema {
    /// List of your characters.
    #[serde(rename = "data")]
    pub data: Vec<models::CharacterSchema>,
}

impl MyCharactersListSchema {
    pub fn new(data: Vec<models::CharacterSchema>) -> MyCharactersListSchema {
        MyCharactersListSchema { data }
    }
}

impl crate::traits::IntoData for MyCharactersListSchema {
    type Data = Vec<models::CharacterSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
