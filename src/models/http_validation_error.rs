use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpValidationError {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<models::ValidationError>>,
}

impl HttpValidationError {
    pub fn new() -> HttpValidationError {
        HttpValidationError { detail: None }
    }
}
