use serde::{Deserialize, Serialize};

use crate::DamageType;

/// The attack style of a [combat option](https://oldschool.runescape.wiki/w/Combat_Options).
///
/// Attack styles determine stat bonuses and experience rewards.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttackStyle {
    /// Accurate attack style. Applies for melee, ranged and magic.
    Accurate,
    /// Aggressive attack style. Applies for melee.
    Aggressive,
    /// Defensive attack style. Applies for melee.
    Defensive,
    /// Controlled attack style. Applies for melee.
    Controlled,
    /// Rapid attack style. Applies for ranged.
    Rapid,
    /// Long-range attack style. Applies for ranged and magic.
    LongRange,
    /// Auto-cast attack style. Applies for magic.
    AutoCast,
    /// Defensive auto-cast attack style. Applies for magic.
    DefensiveAutoCast,
}

/// [Combat option](https://oldschool.runescape.wiki/w/Combat_Options) for a weapon.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CombatOption {
    /// Name of the option.
    pub name: String,
    /// Style of the option.
    pub style: AttackStyle,
    /// Damage type of the option.
    pub damage_type: DamageType,
}

/// Additional data for weapons.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponData {
    /// Delay between attacks in game-ticks.
    pub attack_delay: u8,
    /// Available combat options.
    pub combat_options: Vec<CombatOption>,
}
