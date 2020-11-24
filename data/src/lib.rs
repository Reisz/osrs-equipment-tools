#![deny(missing_docs)]

//! Item data format used by [osrs-equipment-tools](../osrs_equipment_tools/index.html).

mod database;

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use trailblazer::{bool_expr::BoolExpr, vars::Region};

pub use database::*;

/// Main item data struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// The item's in-game name.
    pub name: String,
    /// True if the item is only useable in members worlds.
    pub members: bool,
    /// The item's weight in kilograms.
    pub weight: f32,
    /// The item's wiki URL.
    pub wiki_url: String,
    /// The item's eqipment stats.
    pub stats: Stats,
    /// Equipment slot used by the item.
    pub slot: Slot,
    /// Requirements to equip the item.
    pub requirements: Vec<Requirement>,
    /// For weapons.
    pub weapon: Option<WeaponData>,
    /// Trailblazer requirements
    pub trailblazer: Option<BoolExpr<Region>>,
}

/// Equipment slot.
///
/// Serde renames are for the [OSRSBox](https://www.osrsbox.com/) API format.
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, IntoEnumIterator)]
pub enum Slot {
    /// Head slot
    Head,
    /// Cape slot
    Cape,
    /// Neck slot
    Neck,
    #[serde(rename = "ammo")]
    /// Ammunition slot
    Ammunition,
    /// Weapon slot
    Weapon,
    /// Shield slot
    Shield,
    /// Body slot
    Body,
    /// Legs slot
    Legs,
    /// Hands slot
    Hands,
    /// Feet slot
    Feet,
    /// Ring slot
    Ring,
    #[serde(rename = "2h")]
    /// Two handed item, occupies weapon and shield slot
    TwoHanded,
}

/// Equipment stats of an [`Item`].
///
/// Most values are of type [`i16`] to allow representing aggregate values for full equipment sets.
#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    /// Offensive stats.
    pub attack: BaseStats,
    /// Defensive stats.
    pub defence: BaseStats,
    /// Melee strength bonus.
    pub melee_strength: i16,
    /// Ranged strength bonus.
    pub ranged_strength: i16,
    /// Magic damage bonus in percent.
    ///
    /// Currently no equipment combination can exceed [`u8::MAX`].
    pub magic_damage: u8,
    /// Prayer bonus.
    pub prayer: i16,
}

/// Base bonuses for attack or defence, part of [`Stats`].
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStats {
    /// Bonus for stab damage.
    pub stab: i16,
    /// Bonus for slash damage.
    pub slash: i16,
    /// Bonus for crush damage.
    pub crush: i16,
    /// Bonus for magic damage.
    pub magic: i16,
    /// Bonus for ranged damage.
    pub ranged: i16,
}

/// Requirement to equip an [`Item`].
#[derive(Debug, Serialize, Deserialize)]
pub struct Requirement {
    /// Name of the skill.
    pub skill: String, // TODO create enum
    /// Required level in the skill.
    pub level: u8,
}

/// Data pertaining weapons. Used in [`Item`].
#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponData {
    /// Delay between attacks in ticks.
    pub attack_delay: u8,
}
