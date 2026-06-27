use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DataPageGeOrderSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::GeOrderSchema>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "size")]
    pub size: u32,
    #[serde(rename = "pages")]
    pub pages: u32,
}

impl DataPageGeOrderSchema {
    pub fn new(
        data: Vec<models::GeOrderSchema>,
        total: u32,
        page: u32,
        size: u32,
        pages: u32,
    ) -> DataPageGeOrderSchema {
        DataPageGeOrderSchema {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}
