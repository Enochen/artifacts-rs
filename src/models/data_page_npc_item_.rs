use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DataPageNpcItem {
    #[serde(rename = "data")]
    pub data: Vec<models::NpcItem>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<u32>,
}

impl DataPageNpcItem {
    pub fn new(data: Vec<models::NpcItem>) -> DataPageNpcItem {
        DataPageNpcItem {
            data,
            total: None,
            page: None,
            size: None,
            pages: None,
        }
    }
}
