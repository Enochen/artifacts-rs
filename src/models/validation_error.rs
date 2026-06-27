use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<models::ValidationErrorLocInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(
        rename = "input",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "specta", specta(type = Option<Option<specta_util::Unknown>>))]
    pub input: Option<Option<serde_json::Value>>,
    #[serde(rename = "ctx", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(type = Option<specta_util::Unknown>))]
    pub ctx: Option<serde_json::Value>,
}

impl ValidationError {
    pub fn new(
        loc: Vec<models::ValidationErrorLocInner>,
        msg: String,
        r#type: String,
    ) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
            input: None,
            ctx: None,
        }
    }
}
