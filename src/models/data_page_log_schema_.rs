use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPageLogSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::LogSchema>,
    #[serde(rename = "total", deserialize_with = "Option::deserialize")]
    pub total: Option<u32>,
    #[serde(rename = "page", deserialize_with = "Option::deserialize")]
    pub page: Option<u32>,
    #[serde(rename = "size", deserialize_with = "Option::deserialize")]
    pub size: Option<u32>,
    #[serde(
        rename = "pages",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pages: Option<Option<u32>>,
}

impl DataPageLogSchema {
    pub fn new(
        data: Vec<models::LogSchema>,
        total: Option<u32>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> DataPageLogSchema {
        DataPageLogSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}
