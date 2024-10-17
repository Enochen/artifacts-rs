use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleItemSchema {
    /// Item information.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
    #[serde(
        rename = "ge",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ge: Option<Option<Box<models::GeItemSchema>>>,
}

impl SingleItemSchema {
    pub fn new(item: models::ItemSchema) -> SingleItemSchema {
        SingleItemSchema {
            item: Box::new(item),
            ge: None,
        }
    }
}
