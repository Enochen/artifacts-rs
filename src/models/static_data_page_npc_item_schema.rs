use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StaticDataPageNpcItemSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::NpcItemSchema>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "size")]
    pub size: u32,
    #[serde(rename = "pages")]
    pub pages: u32,
}

impl StaticDataPageNpcItemSchema {
    pub fn new(
        data: Vec<models::NpcItemSchema>,
        total: u32,
        page: u32,
        size: u32,
        pages: u32,
    ) -> StaticDataPageNpcItemSchema {
        StaticDataPageNpcItemSchema {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}
