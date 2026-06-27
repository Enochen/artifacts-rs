use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MemberTokenSubscriptionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MemberTokenSubscriptionResponseDataSchema>,
}

impl MemberTokenSubscriptionResponseSchema {
    pub fn new(
        data: models::MemberTokenSubscriptionResponseDataSchema,
    ) -> MemberTokenSubscriptionResponseSchema {
        MemberTokenSubscriptionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for MemberTokenSubscriptionResponseSchema {
    type Data = Box<models::MemberTokenSubscriptionResponseDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
