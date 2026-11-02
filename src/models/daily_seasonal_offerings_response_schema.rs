use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DailySeasonalOfferingsResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::DailySeasonalOfferingsSchema>,
}

impl DailySeasonalOfferingsResponseSchema {
    pub fn new(data: models::DailySeasonalOfferingsSchema) -> DailySeasonalOfferingsResponseSchema {
        DailySeasonalOfferingsResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for DailySeasonalOfferingsResponseSchema {
    type Data = Box<models::DailySeasonalOfferingsSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
