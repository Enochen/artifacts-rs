use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NpcSchema {
    /// Name of the NPC.
    #[serde(rename = "name")]
    pub name: String,
    /// The code of the NPC. This is the NPC's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Description of the NPC.
    #[serde(rename = "description")]
    pub description: String,
    /// Type of the NPC.
    #[serde(rename = "type")]
    pub r#type: models::NpcType,
}

impl NpcSchema {
    pub fn new(
        name: String,
        code: String,
        description: String,
        r#type: models::NpcType,
    ) -> NpcSchema {
        NpcSchema {
            name,
            code,
            description,
            r#type,
        }
    }
}
