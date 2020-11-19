use std::{collections::HashMap, ops::Index};

use enum_iterator::IntoEnumIterator;
use serde::Deserialize;
use serde_with::with_prefix;

use super::AttackType;

/// Stats for equippable items.
#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    #[serde(flatten, with = "prefix_attack")]
    attack: BaseStats,
    #[serde(flatten, with = "prefix_defence")]
    defence: BaseStats,
    melee_strength: i16,
    ranged_strength: i16,
    magic_damage: i16,
    prayer: i16,
    slot: Slot,
    requirements: HashMap<String, u8>,
}

impl Stats {
    /// Get a reference to the attack stats.
    pub fn attack(&self) -> &BaseStats {
        &self.attack
    }

    /// Get a reference to the defence stats.
    pub fn defence(&self) -> &BaseStats {
        &self.defence
    }
}

impl Index<Stat> for Stats {
    type Output = i16;

    fn index(&self, stat: Stat) -> &i16 {
        match stat {
            Stat::Attack(damage_type) => &self.attack[damage_type],
            Stat::Defence(damage_type) => &self.defence[damage_type],
            Stat::MeleeStrength => &self.melee_strength,
            Stat::RangedStrength => &self.ranged_strength,
            Stat::MagicDamage => &self.magic_damage,
            Stat::Prayer => &self.prayer,
        }
    }
}

with_prefix!(prefix_attack "attack_");
with_prefix!(prefix_defence "defence_");

/// One stat value for each damage type.
#[derive(Debug, Clone, Deserialize)]
pub struct BaseStats {
    stab: i16,
    slash: i16,
    crush: i16,
    magic: i16,
    ranged: i16,
}

impl BaseStats {
    /// Average of all damage types.
    pub fn avg(&self) -> f32 {
        let mut sum = 0.0;

        sum += self.stab as f32;
        sum += self.slash as f32;
        sum += self.crush as f32;
        sum += self.magic as f32;
        sum += self.ranged as f32;

        sum / 5.0
    }

    /// Median of all damage types.
    pub fn median(&self) -> i16 {
        let mut values = [self.stab, self.slash, self.crush, self.magic, self.ranged];
        values.sort_unstable();
        values[2]
    }

    /// Average of the three melee damage types.
    pub fn melee_avg(&self) -> f32 {
        let mut sum = 0.0;

        sum += self.stab as f32;
        sum += self.slash as f32;
        sum += self.crush as f32;

        sum / 3.0
    }
}

impl Index<AttackType> for BaseStats {
    type Output = i16;

    fn index(&self, damage_type: AttackType) -> &i16 {
        match damage_type {
            AttackType::Stab => &self.stab,
            AttackType::Slash => &self.slash,
            AttackType::Crush => &self.crush,
            AttackType::Magic => &self.magic,
            AttackType::Ranged => &self.ranged,
        }
    }
}

/// Available [equipment slots](https://oldschool.runescape.wiki/w/Worn_Equipment). Weapon and TwoHanded are separate.
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, IntoEnumIterator)]
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

/// Enum of all equipment stats for querying [Stats](struct.Stats.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    /// Attack stats based on damage type
    Attack(AttackType),
    /// Defence stats based on damage type
    Defence(AttackType),
    /// Melee strength bonus
    MeleeStrength,
    /// Ranged strength bonus
    RangedStrength,
    /// Magic damage boost (in percent)
    MagicDamage,
    /// Prayer bonus
    Prayer,
}
