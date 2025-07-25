use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MonsterResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MonsterSchema>,
}

impl MonsterResponseSchema {
    pub fn new(data: models::MonsterSchema) -> MonsterResponseSchema {
        MonsterResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for MonsterResponseSchema {
    type Data = Box<models::MonsterSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
