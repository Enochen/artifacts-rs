use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DataPageAccountLeaderboardSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::AccountLeaderboardSchema>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "size")]
    pub size: u32,
    #[serde(rename = "pages")]
    pub pages: u32,
}

impl DataPageAccountLeaderboardSchema {
    pub fn new(
        data: Vec<models::AccountLeaderboardSchema>,
        total: u32,
        page: u32,
        size: u32,
        pages: u32,
    ) -> DataPageAccountLeaderboardSchema {
        DataPageAccountLeaderboardSchema {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}
