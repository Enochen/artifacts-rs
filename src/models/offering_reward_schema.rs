use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct OfferingRewardSchema {
    #[serde(rename = "category")]
    pub category: Category,
    /// XP granted immediately.
    #[serde(rename = "xp")]
    pub xp: u32,
}

impl OfferingRewardSchema {
    pub fn new(category: Category, xp: u32) -> OfferingRewardSchema {
        OfferingRewardSchema { category, xp }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum Category {
    #[serde(rename = "combat")]
    #[default]
    Combat,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "woodcutting")]
    Woodcutting,
    #[serde(rename = "fishing")]
    Fishing,
    #[serde(rename = "alchemy")]
    Alchemy,
    #[serde(rename = "weaponcrafting")]
    Weaponcrafting,
    #[serde(rename = "gearcrafting")]
    Gearcrafting,
    #[serde(rename = "jewelrycrafting")]
    Jewelrycrafting,
    #[serde(rename = "cooking")]
    Cooking,
}
