use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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
    /// Season rewards with required achievement points, sorted by points ascending.
    #[serde(rename = "rewards")]
    pub rewards: Vec<models::StatusSeasonRewardSchema>,
}

impl SeasonSchema {
    pub fn new(rewards: Vec<models::StatusSeasonRewardSchema>) -> SeasonSchema {
        SeasonSchema {
            name: None,
            number: None,
            start_date: None,
            rewards,
        }
    }
}
