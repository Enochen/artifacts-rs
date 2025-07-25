use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MonsterSchema {
    /// Name of the monster.
    #[serde(rename = "name")]
    pub name: String,
    /// The code of the monster. This is the monster's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Monster level.
    #[serde(rename = "level")]
    pub level: i32,
    /// Monster hit points.
    #[serde(rename = "hp")]
    pub hp: i32,
    /// Monster fire attack.
    #[serde(rename = "attack_fire")]
    pub attack_fire: i32,
    /// Monster earth attack.
    #[serde(rename = "attack_earth")]
    pub attack_earth: i32,
    /// Monster water attack.
    #[serde(rename = "attack_water")]
    pub attack_water: i32,
    /// Monster air attack.
    #[serde(rename = "attack_air")]
    pub attack_air: i32,
    /// Monster % fire resistance.
    #[serde(rename = "res_fire")]
    pub res_fire: i32,
    /// Monster % earth resistance.
    #[serde(rename = "res_earth")]
    pub res_earth: i32,
    /// Monster % water resistance.
    #[serde(rename = "res_water")]
    pub res_water: i32,
    /// Monster % air resistance.
    #[serde(rename = "res_air")]
    pub res_air: i32,
    /// Monster % critical strike.
    #[serde(rename = "critical_strike")]
    pub critical_strike: i32,
    /// List of effects.
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<models::SimpleEffectSchema>>,
    /// Monster minimum gold drop.
    #[serde(rename = "min_gold")]
    pub min_gold: i32,
    /// Monster maximum gold drop.
    #[serde(rename = "max_gold")]
    pub max_gold: i32,
    /// Monster drops. This is a list of items that the monster drops after killing the monster.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropRateSchema>,
}

impl MonsterSchema {
    pub fn new(
        name: String,
        code: String,
        level: i32,
        hp: i32,
        attack_fire: i32,
        attack_earth: i32,
        attack_water: i32,
        attack_air: i32,
        res_fire: i32,
        res_earth: i32,
        res_water: i32,
        res_air: i32,
        critical_strike: i32,
        min_gold: i32,
        max_gold: i32,
        drops: Vec<models::DropRateSchema>,
    ) -> MonsterSchema {
        MonsterSchema {
            name,
            code,
            level,
            hp,
            attack_fire,
            attack_earth,
            attack_water,
            attack_air,
            res_fire,
            res_earth,
            res_water,
            res_air,
            critical_strike,
            effects: None,
            min_gold,
            max_gold,
            drops,
        }
    }
}
