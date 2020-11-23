//! Data format used by the [OSRSBox](https://www.osrsbox.com/) API.

use std::{collections::HashMap, convert::TryInto};

use data::{BaseStats, Item, Requirement, Slot, Stats, WeaponData};
use serde::Deserialize;

/// [OSRSBox](https://www.osrsbox.com/) [`ItemProperties`](https://www.osrsbox.com/projects/osrsbox-db/#item-properties).
#[derive(Deserialize)]
pub struct ItemProperties {
    /// Unique OSRS item ID number.
    pub id: u32,
    /// The name of the item.
    pub name: String,
    /// If the item has incomplete wiki data.
    pub incomplete: bool,
    /// If the item is a members-only.
    pub members: bool,
    /// If the item is tradeable (between players and on the GE).
    pub tradeable: bool,
    /// If the item is tradeable (only on GE).
    pub tradeable_on_ge: bool,
    /// If the item is stackable (in inventory).
    pub stackable: bool,
    /// If the item is stacked, indicated by the stack count.
    pub stacked: Option<u32>,
    /// If the item is noted.
    pub noted: bool,
    /// If the item is noteable.
    pub noteable: bool,
    /// The linked ID of the actual item (if noted/placeholder).
    pub linked_id_item: Option<u32>,
    /// The linked ID of an item in noted form.
    pub linked_id_noted: Option<u32>,
    /// The linked ID of an item in placeholder form.
    pub linked_id_placeholder: Option<u32>,
    /// If the item is a placeholder.
    pub placeholder: bool,
    /// If the item is equipable (based on right-click menu entry).
    pub equipable: bool,
    /// If the item is equipable in-game by a player.
    pub equipable_by_player: bool,
    /// If the item is an equipable weapon.
    pub equipable_weapon: bool,
    /// The store price of an item.
    pub cost: u32,
    /// The low alchemy value of the item (cost * 0.4).
    pub lowalch: Option<u32>,
    /// The high alchemy value of the item (cost * 0.6).
    pub highalch: Option<u32>,
    /// The weight (in kilograms) of the item.
    pub weight: Option<f32>,
    /// The Grand Exchange buy limit of the item.
    pub buy_limit: Option<u32>,
    /// If the item is associated with a quest.
    pub quest_item: bool,
    /// Date the item was released (in ISO8601 format).
    pub release_date: Option<String>,
    /// If the item is a duplicate.
    pub duplicate: bool,
    /// The examine text for the item.
    pub examine: Option<String>,
    /// The item icon icon(in base64 encoding).
    pub icon: String,
    /// The OSRS Wiki name for the item.
    pub wiki_name: Option<String>,
    /// The OSRS Wiki URL (possibly including anchor link).
    pub wiki_url: Option<String>,
    /// The OSRS Wiki Exchange URL.
    pub wiki_exchnage: Option<String>,
    /// The equipment bonuses of equipable armour/weapons.
    pub equipment: Option<ItemEquipment>,
    /// The weapon bonuses including attack speed, type and stance.
    pub weapon: Option<ItemWeapon>,
}

impl ItemProperties {
    /// Project the data to the osrs-equipment-tools [`Item`] type.
    pub fn project(self) -> Result<Item, String> {
        assert!(!self.duplicate);

        let mut equipment = self.equipment.ok_or("Missing equipment stats.")?;
        let slot = equipment.slot.take().unwrap();
        let requirements = equipment
            .requirements
            .take()
            .map(|requirements| {
                requirements
                    .into_iter()
                    .map(|(skill, level)| Requirement { skill, level })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Item {
            name: self.name,
            members: self.members,
            weight: self.weight.ok_or("Missing weight.")?,
            wiki_url: self.wiki_url.ok_or("Missing wiki URL.")?,
            stats: equipment.project()?,
            slot,
            requirements,
            weapon: self.weapon.map(|w| w.project()),
        })
    }
}

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
    ///
    /// Not nullable, but option needed for projection.
    pub slot: Option<Slot>,
    /// An object of requirements {skill: level}.
    pub requirements: Option<HashMap<String, u8>>,
}

impl ItemEquipment {
    /// Project the data to the osrs-equipment-tools [`Stats`] type.
    pub fn project(self) -> Result<Stats, String> {
        Ok(Stats {
            attack: BaseStats {
                stab: self.attack_stab,
                slash: self.attack_slash,
                crush: self.attack_crush,
                magic: self.attack_magic,
                ranged: self.attack_ranged,
            },
            defence: BaseStats {
                stab: self.defence_stab,
                slash: self.defence_slash,
                crush: self.defence_crush,
                magic: self.defence_magic,
                ranged: self.defence_ranged,
            },
            melee_strength: self.melee_strength,
            ranged_strength: self.ranged_strength,
            magic_damage: self
                .magic_damage
                .try_into()
                .map_err(|e| format!("Magic damage: {}", e))?,
            prayer: self.prayer,
        })
    }
}

/// [OSRSBox](https://www.osrsbox.com/) [`ItemWeapon`](https://www.osrsbox.com/projects/osrsbox-db/#item-weapon).
#[derive(Deserialize)]
pub struct ItemWeapon {
    /// The attack speed of a weapon (in game ticks).
    pub attack_speed: u8,
    /// The weapon classification (e.g., axes)
    pub weapon_type: String,
    /// An array of weapon stance information.
    pub stances: Vec<ItemWeaponStance>,
}

impl ItemWeapon {
    /// Project the data to the osrs-equipment-tools [`WeaponData`] type.
    pub fn project(self) -> WeaponData {
        WeaponData {
            attack_delay: self.attack_speed,
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) `ItemWeaponStance` (see [Swagger UI](https://api.osrsbox.com/swaggerui)).
#[derive(Deserialize)]
pub struct ItemWeaponStance {
    /// The name of the stance displayed in the interface.
    pub combat_style: String,
    /// The type of damage dealt by the attack.
    pub attack_type: Option<String>,
    /// The attack style as displayed on the tooltip in the interface.
    pub attack_style: Option<String>,
    /// The types of experience gained by using this stance.
    pub experience: Option<String>,
    /// The invisible boosts from the stance.
    pub boosts: Option<String>,
}
