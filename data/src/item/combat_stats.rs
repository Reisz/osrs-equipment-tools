use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

use crate::DamageTypeStats;

/// Equipment stats of an [`Item`](crate::Item).
///
/// Most values are of type [`i16`] to allow representing aggregate values for full equipment sets.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CombatStats {
    /// Offensive stats.
    pub attack: DamageTypeStats,
    /// Defensive stats.
    pub defence: DamageTypeStats,
    /// Melee strength bonus.
    pub melee_strength: i16,
    /// Ranged strength bonus.
    pub ranged_strength: i16,
    /// Magic damage bonus in percent.
    pub magic_damage: i16,
    /// Prayer bonus.
    pub prayer: i16,
}

impl AddAssign<&Self> for CombatStats {
    fn add_assign(&mut self, other: &Self) {
        self.attack += other.attack;
        self.defence += other.defence;
        self.melee_strength += other.melee_strength;
        self.ranged_strength += other.ranged_strength;
        self.magic_damage += other.magic_damage;
        self.prayer += other.prayer;
    }
}
