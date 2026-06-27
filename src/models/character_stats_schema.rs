use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterStatsSchema {
    #[serde(rename = "monsters_killed", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub monsters_killed: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "resources_gathered", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub resources_gathered: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "action_counts", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub action_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "deaths", skip_serializing_if = "Option::is_none")]
    pub deaths: Option<i32>,
}

impl CharacterStatsSchema {
    pub fn new() -> CharacterStatsSchema {
        CharacterStatsSchema {
            monsters_killed: None,
            resources_gathered: None,
            action_counts: None,
            deaths: None,
        }
    }
}
