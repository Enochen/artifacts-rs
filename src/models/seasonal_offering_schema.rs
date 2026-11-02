use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SeasonalOfferingSchema {
    /// Daily offering identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Combat or skill category.
    #[serde(rename = "category")]
    pub category: Category,
    /// Offering source type.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Minimum level of the offering bracket.
    #[serde(rename = "level_min")]
    pub level_min: u32,
    /// Maximum level of the offering bracket.
    #[serde(rename = "level_max")]
    pub level_max: u32,
    #[serde(rename = "requirement")]
    pub requirement: Box<models::OfferingRequirementSchema>,
    #[serde(rename = "favor")]
    pub favor: Box<models::OfferingFavorSchema>,
}

impl SeasonalOfferingSchema {
    pub fn new(
        id: String,
        category: Category,
        r#type: Type,
        level_min: u32,
        level_max: u32,
        requirement: models::OfferingRequirementSchema,
        favor: models::OfferingFavorSchema,
    ) -> SeasonalOfferingSchema {
        SeasonalOfferingSchema {
            id,
            category,
            r#type,
            level_min,
            level_max,
            requirement: Box::new(requirement),
            favor: Box::new(favor),
        }
    }
}
/// Combat or skill category.
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

/// Offering source type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum Type {
    #[serde(rename = "monster")]
    #[default]
    Monster,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "craft")]
    Craft,
}
