use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CombatSimulationResponseSchema {
    /// Combat simulation results.
    #[serde(rename = "data")]
    pub data: Box<models::CombatSimulationDataSchema>,
}

impl CombatSimulationResponseSchema {
    pub fn new(data: models::CombatSimulationDataSchema) -> CombatSimulationResponseSchema {
        CombatSimulationResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CombatSimulationResponseSchema {
    type Data = Box<models::CombatSimulationDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
