use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AcceptOfferingRequestSchema {
    /// Offering identifier to complete.
    #[serde(rename = "offering_id")]
    pub offering_id: String,
}

impl AcceptOfferingRequestSchema {
    pub fn new(offering_id: String) -> AcceptOfferingRequestSchema {
        AcceptOfferingRequestSchema { offering_id }
    }
}
