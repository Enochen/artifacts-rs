use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StaticDataPageSkinSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::SkinSchema>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "size")]
    pub size: u32,
    #[serde(rename = "pages")]
    pub pages: u32,
}

impl StaticDataPageSkinSchema {
    pub fn new(
        data: Vec<models::SkinSchema>,
        total: u32,
        page: u32,
        size: u32,
        pages: u32,
    ) -> StaticDataPageSkinSchema {
        StaticDataPageSkinSchema {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}
