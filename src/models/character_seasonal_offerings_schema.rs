use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterSeasonalOfferingsSchema {
    /// UTC date of this rotation.
    #[serde(rename = "date")]
    pub date: String,
    /// Next daily reset at midnight UTC.
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    /// Two skills selected for this rotation.
    #[serde(rename = "skills")]
    pub skills: Vec<String>,
    /// Percentage of the current level XP requirement granted on completion.
    #[serde(rename = "xp_reward_percent", skip_serializing_if = "Option::is_none")]
    pub xp_reward_percent: Option<i32>,
    #[serde(rename = "offerings")]
    pub offerings: Vec<models::SeasonalOfferingSchema>,
    /// Offerings accepted or completed by this character today.
    #[serde(rename = "selected_offerings")]
    pub selected_offerings: Vec<models::SelectedOfferingSchema>,
    /// Categories already completed by this character today.
    #[serde(rename = "completed_categories")]
    pub completed_categories: Vec<String>,
    /// XP favors active for this character until reset.
    #[serde(rename = "active_favors")]
    pub active_favors: Vec<models::OfferingFavorSchema>,
}

impl CharacterSeasonalOfferingsSchema {
    pub fn new(
        date: String,
        expires_at: String,
        skills: Vec<String>,
        offerings: Vec<models::SeasonalOfferingSchema>,
        selected_offerings: Vec<models::SelectedOfferingSchema>,
        completed_categories: Vec<String>,
        active_favors: Vec<models::OfferingFavorSchema>,
    ) -> CharacterSeasonalOfferingsSchema {
        CharacterSeasonalOfferingsSchema {
            date,
            expires_at,
            skills,
            xp_reward_percent: None,
            offerings,
            selected_offerings,
            completed_categories,
            active_favors,
        }
    }
}
