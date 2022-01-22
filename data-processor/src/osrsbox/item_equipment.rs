use std::collections::HashMap;

use data::{CombatStats, DamageTypeStats, EquipSlot, Skill};
use serde::Deserialize;

/// [OSRSBox](https://www.osrsbox.com/) [`ItemEquipment`](https://www.osrsbox.com/projects/osrsbox-db/#item-equipment).
#[derive(Deserialize)]
pub struct ItemEquipment {
    /// The attack stab bonus of the item.
    pub attack_stab: i16,
    /// The attack slash bonus of the item.
    pub attack_slash: i16,
    /// The attack crush bonus of the item.
    pub attack_crush: i16,
    /// The attack magic bonus of the item.
    pub attack_magic: i16,
    /// The attack ranged bonus of the item.
    pub attack_ranged: i16,
    /// The defence stab bonus of the item.
    pub defence_stab: i16,
    /// The defence slash bonus of the item.
    pub defence_slash: i16,
    /// The defence crush bonus of the item.
    pub defence_crush: i16,
    /// The defence magic bonus of the item.
    pub defence_magic: i16,
    /// The defence ranged bonus of the item.
    pub defence_ranged: i16,
    /// The melee strength bonus of the item.
    pub melee_strength: i16,
    /// The ranged strength bonus of the item.
    pub ranged_strength: i16,
    /// The magic damage bonus of the item.
    pub magic_damage: i16,
    /// The prayer bonus of the item.
    pub prayer: i16,
    /// The equipment slot associated with the item (e.g., head).
    pub slot: EquipSlot,
    /// An object of requirements {skill: level}.
    pub requirements: Option<HashMap<Skill, u8>>,
}

impl From<ItemEquipment> for CombatStats {
    fn from(equipment: ItemEquipment) -> Self {
        Self {
            attack: DamageTypeStats::new(&[
                equipment.attack_stab,
                equipment.attack_slash,
                equipment.attack_crush,
                equipment.attack_magic,
                equipment.attack_ranged,
            ]),
            defence: DamageTypeStats::new(&[
                equipment.defence_stab,
                equipment.defence_slash,
                equipment.defence_crush,
                equipment.defence_magic,
                equipment.defence_ranged,
            ]),
            melee_strength: equipment.melee_strength,
            ranged_strength: equipment.ranged_strength,
            magic_damage: equipment.magic_damage,
            prayer: equipment.prayer,
        }
    }
}

impl ItemEquipment {
    /// Returns true if at least one stat value is greater than zero.
    #[must_use]
    pub fn has_positive(&self) -> bool {
        self.attack_stab > 0
            || self.attack_slash > 0
            || self.attack_crush > 0
            || self.attack_magic > 0
            || self.attack_ranged > 0
            || self.defence_stab > 0
            || self.defence_slash > 0
            || self.defence_crush > 0
            || self.defence_magic > 0
            || self.defence_ranged > 0
            || self.melee_strength > 0
            || self.ranged_strength > 0
            || self.magic_damage > 0
            || self.prayer > 0
    }
}
