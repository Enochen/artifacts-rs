use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RaidResponseSchema {
    /// Raid details.
    #[serde(rename = "data")]
    pub data: Box<models::RaidSchema>,
}

impl RaidResponseSchema {
    pub fn new(data: models::RaidSchema) -> RaidResponseSchema {
        RaidResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RaidResponseSchema {
    type Data = Box<models::RaidSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
