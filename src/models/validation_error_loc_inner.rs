use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ValidationErrorLocInner {}

impl ValidationErrorLocInner {
    pub fn new() -> ValidationErrorLocInner {
        ValidationErrorLocInner {}
    }
}
