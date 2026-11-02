use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SelectedOfferingSchema {
    #[serde(rename = "offering_id")]
    pub offering_id: String,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "accepted_at")]
    pub accepted_at: String,
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "reward_xp", skip_serializing_if = "Option::is_none")]
    pub reward_xp: Option<u32>,
    #[serde(rename = "requirement")]
    pub requirement: Box<models::OfferingRequirementSchema>,
    #[serde(rename = "favor")]
    pub favor: Box<models::OfferingFavorSchema>,
    #[serde(rename = "expires_at")]
    pub expires_at: String,
}

impl SelectedOfferingSchema {
    pub fn new(
        offering_id: String,
        category: Category,
        status: Status,
        accepted_at: String,
        requirement: models::OfferingRequirementSchema,
        favor: models::OfferingFavorSchema,
        expires_at: String,
    ) -> SelectedOfferingSchema {
        SelectedOfferingSchema {
            offering_id,
            category,
            status,
            accepted_at,
            completed_at: None,
            reward_xp: None,
            requirement: Box::new(requirement),
            favor: Box::new(favor),
            expires_at,
        }
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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum Status {
    #[serde(rename = "accepted")]
    #[default]
    Accepted,
    #[serde(rename = "completed")]
    Completed,
}
