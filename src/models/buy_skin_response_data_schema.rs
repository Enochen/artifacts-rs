use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BuySkinResponseDataSchema {
    /// Updated list of owned skins.
    #[serde(rename = "skins")]
    pub skins: Vec<String>,
    /// Code of the purchased skin.
    #[serde(rename = "skin")]
    pub skin: String,
    /// Remaining gem balance.
    #[serde(rename = "gems")]
    pub gems: i32,
}

impl BuySkinResponseDataSchema {
    pub fn new(skins: Vec<String>, skin: String, gems: i32) -> BuySkinResponseDataSchema {
        BuySkinResponseDataSchema { skins, skin, gems }
    }
}
