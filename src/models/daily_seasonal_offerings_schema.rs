use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DailySeasonalOfferingsSchema {
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
}

impl DailySeasonalOfferingsSchema {
    pub fn new(
        date: String,
        expires_at: String,
        skills: Vec<String>,
        offerings: Vec<models::SeasonalOfferingSchema>,
    ) -> DailySeasonalOfferingsSchema {
        DailySeasonalOfferingsSchema {
            date,
            expires_at,
            skills,
            xp_reward_percent: None,
            offerings,
        }
    }
}
