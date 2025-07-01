use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeasonSchema {
    /// Season name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Season number.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Season start date.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Season badges with required achievement points.
    #[serde(rename = "badges")]
    pub badges: Vec<models::SeasonBadgeSchema>,
    /// Season skins with required achievement points.
    #[serde(rename = "skins")]
    pub skins: Vec<models::SeasonSkinSchema>,
}

impl SeasonSchema {
    pub fn new(
        badges: Vec<models::SeasonBadgeSchema>,
        skins: Vec<models::SeasonSkinSchema>,
    ) -> SeasonSchema {
        SeasonSchema {
            name: None,
            number: None,
            start_date: None,
            badges,
            skins,
        }
    }
}
