use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<models::ValidationErrorLocInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(
        loc: Vec<models::ValidationErrorLocInner>,
        msg: String,
        r#type: String,
    ) -> ValidationError {
        ValidationError { loc, msg, r#type }
    }
}
