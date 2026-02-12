use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LocationInner {}

impl LocationInner {
    pub fn new() -> LocationInner {
        LocationInner {}
    }
}
