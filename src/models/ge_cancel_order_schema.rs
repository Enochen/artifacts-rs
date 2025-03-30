use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeCancelOrderSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
}

impl GeCancelOrderSchema {
    pub fn new(id: String) -> GeCancelOrderSchema {
        GeCancelOrderSchema { id }
    }
}
