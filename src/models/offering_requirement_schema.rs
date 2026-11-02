use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct OfferingRequirementSchema {
    /// Item code required by the offering.
    #[serde(rename = "code")]
    pub code: String,
    /// Required item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl OfferingRequirementSchema {
    pub fn new(code: String, quantity: u32) -> OfferingRequirementSchema {
        OfferingRequirementSchema { code, quantity }
    }
}
