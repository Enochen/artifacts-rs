use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AssistantAnswerDataSchema {
    /// The assistant's answer.
    #[serde(rename = "answer")]
    pub answer: String,
    /// Updated assistant rate limit after this request.
    #[serde(rename = "assistant")]
    pub assistant: Box<models::RateLimitScopeSchema>,
    /// Whether this question cost 1 gem.
    #[serde(rename = "paid_with_gems")]
    pub paid_with_gems: bool,
}

impl AssistantAnswerDataSchema {
    pub fn new(
        answer: String,
        assistant: models::RateLimitScopeSchema,
        paid_with_gems: bool,
    ) -> AssistantAnswerDataSchema {
        AssistantAnswerDataSchema {
            answer,
            assistant: Box::new(assistant),
            paid_with_gems,
        }
    }
}
