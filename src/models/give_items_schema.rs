use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveItemsSchema {
    /// List of items to give
    #[serde(rename = "items")]
    pub items: Vec<models::SimpleItemSchema>,
    /// Character name. The name of the character who will receive the items.
    #[serde(rename = "character")]
    pub character: String,
}

impl GiveItemsSchema {
    pub fn new(items: Vec<models::SimpleItemSchema>, character: String) -> GiveItemsSchema {
        GiveItemsSchema { items, character }
    }
}
