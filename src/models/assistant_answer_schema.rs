use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AssistantAnswerSchema {
    #[serde(rename = "data")]
    pub data: Box<models::AssistantAnswerDataSchema>,
}

impl AssistantAnswerSchema {
    pub fn new(data: models::AssistantAnswerDataSchema) -> AssistantAnswerSchema {
        AssistantAnswerSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for AssistantAnswerSchema {
    type Data = Box<models::AssistantAnswerDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
