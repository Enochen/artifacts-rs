use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterSeasonalOfferingsResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterSeasonalOfferingsSchema>,
}

impl CharacterSeasonalOfferingsResponseSchema {
    pub fn new(
        data: models::CharacterSeasonalOfferingsSchema,
    ) -> CharacterSeasonalOfferingsResponseSchema {
        CharacterSeasonalOfferingsResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CharacterSeasonalOfferingsResponseSchema {
    type Data = Box<models::CharacterSeasonalOfferingsSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
