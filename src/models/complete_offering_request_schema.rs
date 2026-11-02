use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CompleteOfferingRequestSchema {
    /// Offering identifier to complete.
    #[serde(rename = "offering_id")]
    pub offering_id: String,
}

impl CompleteOfferingRequestSchema {
    pub fn new(offering_id: String) -> CompleteOfferingRequestSchema {
        CompleteOfferingRequestSchema { offering_id }
    }
}
