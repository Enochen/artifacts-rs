use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterSchema {
    /// Name of the character.
    #[serde(rename = "name")]
    pub name: String,
    /// Account name.
    #[serde(rename = "account")]
    pub account: String,
    /// Character skin code.
    #[serde(rename = "skin")]
    pub skin: models::CharacterSkin,
    /// Combat level.
    #[serde(rename = "level")]
    pub level: i32,
    /// The current xp level of the combat level.
    #[serde(rename = "xp")]
    pub xp: i32,
    /// XP required to level up the character.
    #[serde(rename = "max_xp")]
    pub max_xp: i32,
    /// The numbers of gold on this character.
    #[serde(rename = "gold")]
    pub gold: i32,
    /// *Not available, on the roadmap. Character movement speed.
    #[serde(rename = "speed")]
    pub speed: i32,
    /// Mining level.
    #[serde(rename = "mining_level")]
    pub mining_level: i32,
    /// The current xp level of the Mining skill.
    #[serde(rename = "mining_xp")]
    pub mining_xp: i32,
    /// Mining XP required to level up the skill.
    #[serde(rename = "mining_max_xp")]
    pub mining_max_xp: i32,
    /// Woodcutting level.
    #[serde(rename = "woodcutting_level")]
    pub woodcutting_level: i32,
    /// The current xp level of the Woodcutting skill.
    #[serde(rename = "woodcutting_xp")]
    pub woodcutting_xp: i32,
    /// Woodcutting XP required to level up the skill.
    #[serde(rename = "woodcutting_max_xp")]
    pub woodcutting_max_xp: i32,
    /// Fishing level.
    #[serde(rename = "fishing_level")]
    pub fishing_level: i32,
    /// The current xp level of the Fishing skill.
    #[serde(rename = "fishing_xp")]
    pub fishing_xp: i32,
    /// Fishing XP required to level up the skill.
    #[serde(rename = "fishing_max_xp")]
    pub fishing_max_xp: i32,
    /// Weaponcrafting level.
    #[serde(rename = "weaponcrafting_level")]
    pub weaponcrafting_level: i32,
    /// The current xp level of the Weaponcrafting skill.
    #[serde(rename = "weaponcrafting_xp")]
    pub weaponcrafting_xp: i32,
    /// Weaponcrafting XP required to level up the skill.
    #[serde(rename = "weaponcrafting_max_xp")]
    pub weaponcrafting_max_xp: i32,
    /// Gearcrafting level.
    #[serde(rename = "gearcrafting_level")]
    pub gearcrafting_level: i32,
    /// The current xp level of the Gearcrafting skill.
    #[serde(rename = "gearcrafting_xp")]
    pub gearcrafting_xp: i32,
    /// Gearcrafting XP required to level up the skill.
    #[serde(rename = "gearcrafting_max_xp")]
    pub gearcrafting_max_xp: i32,
    /// Jewelrycrafting level.
    #[serde(rename = "jewelrycrafting_level")]
    pub jewelrycrafting_level: i32,
    /// The current xp level of the Jewelrycrafting skill.
    #[serde(rename = "jewelrycrafting_xp")]
    pub jewelrycrafting_xp: i32,
    /// Jewelrycrafting XP required to level up the skill.
    #[serde(rename = "jewelrycrafting_max_xp")]
    pub jewelrycrafting_max_xp: i32,
    /// The current xp level of the Cooking skill.
    #[serde(rename = "cooking_level")]
    pub cooking_level: i32,
    /// Cooking XP.
    #[serde(rename = "cooking_xp")]
    pub cooking_xp: i32,
    /// Cooking XP required to level up the skill.
    #[serde(rename = "cooking_max_xp")]
    pub cooking_max_xp: i32,
    /// Alchemy level.
    #[serde(rename = "alchemy_level")]
    pub alchemy_level: i32,
    /// Alchemy XP.
    #[serde(rename = "alchemy_xp")]
    pub alchemy_xp: i32,
    /// Alchemy XP required to level up the skill.
    #[serde(rename = "alchemy_max_xp")]
    pub alchemy_max_xp: i32,
    /// Character actual HP.
    #[serde(rename = "hp")]
    pub hp: i32,
    /// Character max HP.
    #[serde(rename = "max_hp")]
    pub max_hp: i32,
    /// *Increase speed attack (reduce fight cooldown)
    #[serde(rename = "haste")]
    pub haste: i32,
    /// % Critical strike. Critical strikes adds 50% extra damage to an attack (1.5x).
    #[serde(rename = "critical_strike")]
    pub critical_strike: i32,
    /// Wisdom increases the amount of XP gained from fights and skills (1% extra per 10 wisdom).
    #[serde(rename = "wisdom")]
    pub wisdom: i32,
    /// Prospecting increases the chances of getting drops from fights and skills (1% extra per 10 PP).
    #[serde(rename = "prospecting")]
    pub prospecting: i32,
    /// Fire attack.
    #[serde(rename = "attack_fire")]
    pub attack_fire: i32,
    /// Earth attack.
    #[serde(rename = "attack_earth")]
    pub attack_earth: i32,
    /// Water attack.
    #[serde(rename = "attack_water")]
    pub attack_water: i32,
    /// Air attack.
    #[serde(rename = "attack_air")]
    pub attack_air: i32,
    /// % Damage. Damage increases your attack in all elements.
    #[serde(rename = "dmg")]
    pub dmg: i32,
    /// % Fire damage. Damage increases your fire attack.
    #[serde(rename = "dmg_fire")]
    pub dmg_fire: i32,
    /// % Earth damage. Damage increases your earth attack.
    #[serde(rename = "dmg_earth")]
    pub dmg_earth: i32,
    /// % Water damage. Damage increases your water attack.
    #[serde(rename = "dmg_water")]
    pub dmg_water: i32,
    /// % Air damage. Damage increases your air attack.
    #[serde(rename = "dmg_air")]
    pub dmg_air: i32,
    /// % Fire resistance. Reduces fire attack.
    #[serde(rename = "res_fire")]
    pub res_fire: i32,
    /// % Earth resistance. Reduces earth attack.
    #[serde(rename = "res_earth")]
    pub res_earth: i32,
    /// % Water resistance. Reduces water attack.
    #[serde(rename = "res_water")]
    pub res_water: i32,
    /// % Air resistance. Reduces air attack.
    #[serde(rename = "res_air")]
    pub res_air: i32,
    /// Character x coordinate.
    #[serde(rename = "x")]
    pub x: i32,
    /// Character y coordinate.
    #[serde(rename = "y")]
    pub y: i32,
    /// Cooldown in seconds.
    #[serde(rename = "cooldown")]
    pub cooldown: i32,
    /// Datetime Cooldown expiration.
    #[serde(
        rename = "cooldown_expiration",
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_expiration: Option<String>,
    /// Weapon slot.
    #[serde(rename = "weapon_slot")]
    pub weapon_slot: String,
    /// Rune slot.
    #[serde(rename = "rune_slot")]
    pub rune_slot: String,
    /// Shield slot.
    #[serde(rename = "shield_slot")]
    pub shield_slot: String,
    /// Helmet slot.
    #[serde(rename = "helmet_slot")]
    pub helmet_slot: String,
    /// Body armor slot.
    #[serde(rename = "body_armor_slot")]
    pub body_armor_slot: String,
    /// Leg armor slot.
    #[serde(rename = "leg_armor_slot")]
    pub leg_armor_slot: String,
    /// Boots slot.
    #[serde(rename = "boots_slot")]
    pub boots_slot: String,
    /// Ring 1 slot.
    #[serde(rename = "ring1_slot")]
    pub ring1_slot: String,
    /// Ring 2 slot.
    #[serde(rename = "ring2_slot")]
    pub ring2_slot: String,
    /// Amulet slot.
    #[serde(rename = "amulet_slot")]
    pub amulet_slot: String,
    /// Artifact 1 slot.
    #[serde(rename = "artifact1_slot")]
    pub artifact1_slot: String,
    /// Artifact 2 slot.
    #[serde(rename = "artifact2_slot")]
    pub artifact2_slot: String,
    /// Artifact 3 slot.
    #[serde(rename = "artifact3_slot")]
    pub artifact3_slot: String,
    /// Utility 1 slot.
    #[serde(rename = "utility1_slot")]
    pub utility1_slot: String,
    /// Utility 1 quantity.
    #[serde(rename = "utility1_slot_quantity")]
    pub utility1_slot_quantity: u32,
    /// Utility 2 slot.
    #[serde(rename = "utility2_slot")]
    pub utility2_slot: String,
    /// Utility 2 quantity.
    #[serde(rename = "utility2_slot_quantity")]
    pub utility2_slot_quantity: u32,
    /// Bag slot.
    #[serde(rename = "bag_slot")]
    pub bag_slot: String,
    /// Task in progress.
    #[serde(rename = "task")]
    pub task: String,
    /// Task type.
    #[serde(rename = "task_type")]
    pub task_type: String,
    /// Task progression.
    #[serde(rename = "task_progress")]
    pub task_progress: i32,
    /// Task total objective.
    #[serde(rename = "task_total")]
    pub task_total: i32,
    /// Inventory max items.
    #[serde(rename = "inventory_max_items")]
    pub inventory_max_items: i32,
    /// List of inventory slots.
    #[serde(rename = "inventory", skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Vec<models::InventorySlot>>,
}

impl CharacterSchema {
    pub fn new(
        name: String,
        account: String,
        skin: models::CharacterSkin,
        level: i32,
        xp: i32,
        max_xp: i32,
        gold: i32,
        speed: i32,
        mining_level: i32,
        mining_xp: i32,
        mining_max_xp: i32,
        woodcutting_level: i32,
        woodcutting_xp: i32,
        woodcutting_max_xp: i32,
        fishing_level: i32,
        fishing_xp: i32,
        fishing_max_xp: i32,
        weaponcrafting_level: i32,
        weaponcrafting_xp: i32,
        weaponcrafting_max_xp: i32,
        gearcrafting_level: i32,
        gearcrafting_xp: i32,
        gearcrafting_max_xp: i32,
        jewelrycrafting_level: i32,
        jewelrycrafting_xp: i32,
        jewelrycrafting_max_xp: i32,
        cooking_level: i32,
        cooking_xp: i32,
        cooking_max_xp: i32,
        alchemy_level: i32,
        alchemy_xp: i32,
        alchemy_max_xp: i32,
        hp: i32,
        max_hp: i32,
        haste: i32,
        critical_strike: i32,
        wisdom: i32,
        prospecting: i32,
        attack_fire: i32,
        attack_earth: i32,
        attack_water: i32,
        attack_air: i32,
        dmg: i32,
        dmg_fire: i32,
        dmg_earth: i32,
        dmg_water: i32,
        dmg_air: i32,
        res_fire: i32,
        res_earth: i32,
        res_water: i32,
        res_air: i32,
        x: i32,
        y: i32,
        cooldown: i32,
        weapon_slot: String,
        rune_slot: String,
        shield_slot: String,
        helmet_slot: String,
        body_armor_slot: String,
        leg_armor_slot: String,
        boots_slot: String,
        ring1_slot: String,
        ring2_slot: String,
        amulet_slot: String,
        artifact1_slot: String,
        artifact2_slot: String,
        artifact3_slot: String,
        utility1_slot: String,
        utility1_slot_quantity: u32,
        utility2_slot: String,
        utility2_slot_quantity: u32,
        bag_slot: String,
        task: String,
        task_type: String,
        task_progress: i32,
        task_total: i32,
        inventory_max_items: i32,
    ) -> CharacterSchema {
        CharacterSchema {
            name,
            account,
            skin,
            level,
            xp,
            max_xp,
            gold,
            speed,
            mining_level,
            mining_xp,
            mining_max_xp,
            woodcutting_level,
            woodcutting_xp,
            woodcutting_max_xp,
            fishing_level,
            fishing_xp,
            fishing_max_xp,
            weaponcrafting_level,
            weaponcrafting_xp,
            weaponcrafting_max_xp,
            gearcrafting_level,
            gearcrafting_xp,
            gearcrafting_max_xp,
            jewelrycrafting_level,
            jewelrycrafting_xp,
            jewelrycrafting_max_xp,
            cooking_level,
            cooking_xp,
            cooking_max_xp,
            alchemy_level,
            alchemy_xp,
            alchemy_max_xp,
            hp,
            max_hp,
            haste,
            critical_strike,
            wisdom,
            prospecting,
            attack_fire,
            attack_earth,
            attack_water,
            attack_air,
            dmg,
            dmg_fire,
            dmg_earth,
            dmg_water,
            dmg_air,
            res_fire,
            res_earth,
            res_water,
            res_air,
            x,
            y,
            cooldown,
            cooldown_expiration: None,
            weapon_slot,
            rune_slot,
            shield_slot,
            helmet_slot,
            body_armor_slot,
            leg_armor_slot,
            boots_slot,
            ring1_slot,
            ring2_slot,
            amulet_slot,
            artifact1_slot,
            artifact2_slot,
            artifact3_slot,
            utility1_slot,
            utility1_slot_quantity,
            utility2_slot,
            utility2_slot_quantity,
            bag_slot,
            task,
            task_type,
            task_progress,
            task_total,
            inventory_max_items,
            inventory: None,
        }
    }
}
