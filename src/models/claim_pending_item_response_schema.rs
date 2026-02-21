use crate::models;
use serde::{Deserialize, Serialize};

/// ClaimPendingItemResponseSchema : Response schema for claim pending item action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClaimPendingItemResponseSchema {
    /// Response data containing cooldown, item, and character.
    #[serde(rename = "data")]
    pub data: Box<models::ClaimPendingItemDataSchema>,
}

impl ClaimPendingItemResponseSchema {
    /// Response schema for claim pending item action.
    pub fn new(data: models::ClaimPendingItemDataSchema) -> ClaimPendingItemResponseSchema {
        ClaimPendingItemResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for ClaimPendingItemResponseSchema {
    type Data = Box<models::ClaimPendingItemDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
