use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ErrorResponseSchema {
    #[serde(rename = "error")]
    pub error: Box<models::ErrorSchema>,
}

impl ErrorResponseSchema {
    pub fn new(error: models::ErrorSchema) -> ErrorResponseSchema {
        ErrorResponseSchema {
            error: Box::new(error),
        }
    }
}
