use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StaticDataPageSeasonRewardSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::SeasonRewardSchema>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "size")]
    pub size: u32,
    #[serde(rename = "pages")]
    pub pages: u32,
}

impl StaticDataPageSeasonRewardSchema {
    pub fn new(
        data: Vec<models::SeasonRewardSchema>,
        total: u32,
        page: u32,
        size: u32,
        pages: u32,
    ) -> StaticDataPageSeasonRewardSchema {
        StaticDataPageSeasonRewardSchema {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}
