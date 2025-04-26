use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::SkillDataSchema>,
}

impl SkillResponseSchema {
    pub fn new(data: models::SkillDataSchema) -> SkillResponseSchema {
        SkillResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for SkillResponseSchema {
    type Data = Box<models::SkillDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
