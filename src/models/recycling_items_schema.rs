use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RecyclingItemsSchema {
    /// Objects received.
    #[serde(rename = "items")]
    pub items: Vec<models::DropSchema>,
}

impl RecyclingItemsSchema {
    pub fn new(items: Vec<models::DropSchema>) -> RecyclingItemsSchema {
        RecyclingItemsSchema { items }
    }
}
