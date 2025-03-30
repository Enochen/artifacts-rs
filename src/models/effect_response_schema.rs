use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::EffectSchema>,
}

impl EffectResponseSchema {
    pub fn new(data: models::EffectSchema) -> EffectResponseSchema {
        EffectResponseSchema {
            data: Box::new(data),
        }
    }
}
