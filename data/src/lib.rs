#![deny(missing_docs)]

//! Item data format used by [osrs-equipment-tools](../osrs_equipment_tools/index.html).

mod database;

use std::ops::AddAssign;

use enum_iterator::IntoEnumIterator;
#[cfg(feature = "trailblazer")]
use regions::{bool_expr::BoolExpr, vars::Region};
use serde::{Deserialize, Serialize};

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
    /// The item's icon in BMP format.
    pub icon_data: Vec<u8>,
    /// The item's eqipment stats.
    pub stats: Stats,
    /// Equipment slot used by the item.
    pub slot: Slot,
    /// Requirements to equip the item.
    pub requirements: Vec<Requirement>,
    /// For weapons.
    pub weapon: Option<WeaponData>,
    /// The lowest clue tier required to obtain this item
    pub clue: Option<Clue>,
    /// True if the item is a 3rd age piece
    pub third_age: bool,
    /// Trailblazer requirements
    #[cfg(feature = "trailblazer")]
    pub trailblazer: Option<BoolExpr<Region>>,
}

/// Equipment slot.
///
/// Serde renames are for the [OSRSBox](https://www.osrsbox.com/) API format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, IntoEnumIterator)]
#[serde(rename_all = "lowercase")]
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

/// Denotes the available clue tiers.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize)]
pub enum Clue {
    /// Beginner clues
    Beginner,
    /// Easy clues
    Easy,
    /// Medium clues
    Medium,
    /// Hard clues
    Hard,
    /// Elite clues
    Elite,
    /// Master clues
    Master,
}

/// Equipment stats of an [`Item`].
///
/// Most values are of type [`i16`] to allow representing aggregate values for full equipment sets.
#[derive(Debug, Default, Serialize, Deserialize)]
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

impl AddAssign<&Self> for Stats {
    fn add_assign(&mut self, other: &Self) {
        self.attack += &other.attack;
        self.defence += &other.defence;
        self.melee_strength += other.melee_strength;
        self.ranged_strength += other.ranged_strength;
        self.magic_damage += other.magic_damage;
        self.prayer += other.prayer;
    }
}

/// Base bonuses for attack or defence, part of [`Stats`].
#[derive(Debug, Default, Serialize, Deserialize)]
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

impl AddAssign<&Self> for BaseStats {
    fn add_assign(&mut self, other: &Self) {
        self.stab += other.stab;
        self.slash += other.slash;
        self.crush += other.crush;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
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
