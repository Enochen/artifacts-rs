use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationErrorLocInner {}

impl ValidationErrorLocInner {
    pub fn new() -> ValidationErrorLocInner {
        ValidationErrorLocInner {}
    }
}
