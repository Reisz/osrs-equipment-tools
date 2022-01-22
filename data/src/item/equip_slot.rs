use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

/// Equipment slot.
///
/// Serde renames are for the [OSRSBox](https://www.osrsbox.com/) API format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, IntoEnumIterator)]
#[serde(rename_all = "lowercase")]
pub enum EquipSlot {
    /// Head slot.
    Head,
    /// Cape slot.
    Cape,
    /// Neck slot.
    Neck,
    /// Ammunition slot.
    #[serde(rename = "ammo")]
    Ammunition,
    /// Weapon slot.
    Weapon,
    /// Shield slot.
    Shield,
    /// Body slot.
    Body,
    /// Legs slot.
    Legs,
    /// Hands slot.
    Hands,
    /// Feet slot.
    Feet,
    /// Ring slot.
    Ring,
    /// Two handed item. Occupies weapon and shield slot.
    #[serde(rename = "2h")]
    TwoHanded,
}
