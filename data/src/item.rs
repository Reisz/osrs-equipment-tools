use serde::{Deserialize, Serialize};

pub use attainability::*;
pub use combat_stats::*;
pub use equip_slot::*;
pub use requirement::*;
pub use weapon_data::*;

mod attainability;
mod combat_stats;
mod equip_slot;
mod requirement;
mod weapon_data;

/// Main item data struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    /// The item's in-game name.
    pub name: String,
    /// The item's wiki URL.
    pub wiki_url: String,
    /// The item's icon in BMP format.
    pub icon_data: Vec<u8>,
    /// True if the item is only useable in members worlds.
    pub members: bool,
    /// The item's weight in kilograms.
    pub weight: f32,
    /// The item's combat stats.
    pub combat_stats: CombatStats,
    /// Additional data for weapons.
    pub weapon_data: Option<WeaponData>,
    /// Equipment slot used by the item.
    pub equip_slot: EquipSlot,
    /// Requirements to equip the item.
    pub requirements: Vec<Requirement>,
    /// Data about how to obtain the item.
    pub attainability: Attainability,
}
