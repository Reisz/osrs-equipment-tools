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
    pub wiki_exchange: Option<String>,
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
            trailblazer: None,
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

    /// Returns true if at least one stat value is greater than zero.
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

#[cfg(test)]
mod tests {
    use data::Slot;

    use super::*;

    #[test]
    fn test_equipment() {
        let i: ItemProperties = serde_json::from_str(
            r#"{"id": 10828, "name": "Helm of neitiznot", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 10843, "linked_id_placeholder": 17668, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": false, "cost": 50000, "lowalch": 20000, "highalch": 30000, "weight": 2.267, "buy_limit": 70, "quest_item": false, "release_date": "2007-02-06", "duplicate": false, "examine": "A gift from Neitiznot's Burgher.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAACyElEQVR4XtWW70tTURzG/f4JvghEEMYQkbEXIxziaDgUh4aKhflCLeb6IXO5tDWtpRnq1PzBpm2L/NVaSkkjUQQtpfBVRC+iv+jJcy67zOt058TuogcujJ0v5/mc55z7Pbeo6P8WQfvPxZKtlxIhHApB3IQQn5+TqJcWYXtjgxtYDAYBE8K7V3GJeikRYrMv1MlzGyhpbieTegAR1paW1O0Sm5ywEolILEBKhPnxNonJCcvhMD6srkgsQFiEZKybb1d6xdqKs1K2d2FiQh+g6dFmDPXXI5VICBgQUus9mB1rlUhUSoTxx03oc7sFDQivFzoQmZoSgP8rEUb8TrycmRE0IISCV3miYvVSUuIf8HjwJhoVNCCMBRoF05QWYX2xEw0OB7bW14SBngw0CNZK66S5TV6D3+vVrJi9adpHqd9adaGns1M/oMz40+Zs+9iZmgwGOezt7i51LDrTroHPq5QDmjZj9xhrdm9jsVNALJFyo5HXnF5A3kVqPznc2cF+KoWdzU28X77FG19o5CkC/fd5Qs1OJy4VF+t5fpgIi1PX8XVvD0e7uxzq80cPPyfss2L62SiGH/hUILPJhEfeOj2BmAjHB/scigEdpvrwKeE+k1BrUyOsFgt8d+3/BigzoV6XC+0tLai12U5+2/QH+vUtgJ+Hfvz48lAFSkS7+Haya2WwtxY3O6xorKvE0L1q/YF+Hw9zoO8HgyoMa5jxuRvqxXunuwZtTWY89xUgoZoqMyzmChgNpRyIJcP+z3xanHaUlpTwN013oCvVFg5UUW5QYdKNL90w6+1WDpQ5pp0pT1KArBYTB8rsxJk1hrKyQqTDRHDYLnMgc6WRf+ucNaVCbReTAsTOEQPKviVKQtnH8i7lfDCg7IbEOzR7CpAOkwLEzlF2Q/YWVp0Dq4sITkfNBYaEOrvu14VW58GklWtcB+UyyzUuoz9ZOF1UKKk1OwAAAABJRU5ErkJggg==", "wiki_name": "Helm of neitiznot", "wiki_url": "https://oldschool.runescape.wiki/w/Helm_of_neitiznot", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Helm_of_neitiznot", "equipment": {"attack_stab": 0, "attack_slash": 0, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 31, "defence_slash": 29, "defence_crush": 34, "defence_magic": 3, "defence_ranged": 30, "melee_strength": 3, "ranged_strength": 0, "magic_damage": 0, "prayer": 3, "slot": "head", "requirements": {"defence": 55}}, "weapon": null}"#,
        ).unwrap();

        assert_eq!(i.id, 10828);
        assert_eq!(i.name, "Helm of neitiznot");
        assert_eq!(i.incomplete, false);
        assert_eq!(i.members, true);
        assert_eq!(i.tradeable, true);
        assert_eq!(i.tradeable_on_ge, true);
        assert_eq!(i.stackable, false);
        assert!(i.stacked.is_none());
        assert_eq!(i.noted, false);
        assert_eq!(i.noteable, true);
        assert!(i.linked_id_item.is_none());
        assert_eq!(i.linked_id_noted, Some(10843));
        assert_eq!(i.linked_id_placeholder, Some(17668));
        assert_eq!(i.placeholder, false);
        assert_eq!(i.equipable, true);
        assert_eq!(i.equipable_by_player, true);
        assert_eq!(i.equipable_weapon, false);
        assert_eq!(i.cost, 50000);
        assert_eq!(i.lowalch, Some(20000));
        assert_eq!(i.highalch, Some(30000));
        assert_eq!(i.weight, Some(2.267));
        assert_eq!(i.buy_limit, Some(70));
        assert_eq!(i.quest_item, false);
        assert_eq!(i.release_date.as_ref().unwrap(), "2007-02-06");
        assert_eq!(i.duplicate, false);
        assert_eq!(
            i.examine.as_ref().unwrap(),
            "A gift from Neitiznot's Burgher."
        );
        assert_eq!(i.icon, "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAACyElEQVR4XtWW70tTURzG/f4JvghEEMYQkbEXIxziaDgUh4aKhflCLeb6IXO5tDWtpRnq1PzBpm2L/NVaSkkjUQQtpfBVRC+iv+jJcy67zOt058TuogcujJ0v5/mc55z7Pbeo6P8WQfvPxZKtlxIhHApB3IQQn5+TqJcWYXtjgxtYDAYBE8K7V3GJeikRYrMv1MlzGyhpbieTegAR1paW1O0Sm5ywEolILEBKhPnxNonJCcvhMD6srkgsQFiEZKybb1d6xdqKs1K2d2FiQh+g6dFmDPXXI5VICBgQUus9mB1rlUhUSoTxx03oc7sFDQivFzoQmZoSgP8rEUb8TrycmRE0IISCV3miYvVSUuIf8HjwJhoVNCCMBRoF05QWYX2xEw0OB7bW14SBngw0CNZK66S5TV6D3+vVrJi9adpHqd9adaGns1M/oMz40+Zs+9iZmgwGOezt7i51LDrTroHPq5QDmjZj9xhrdm9jsVNALJFyo5HXnF5A3kVqPznc2cF+KoWdzU28X77FG19o5CkC/fd5Qs1OJy4VF+t5fpgIi1PX8XVvD0e7uxzq80cPPyfss2L62SiGH/hUILPJhEfeOj2BmAjHB/scigEdpvrwKeE+k1BrUyOsFgt8d+3/BigzoV6XC+0tLai12U5+2/QH+vUtgJ+Hfvz48lAFSkS7+Haya2WwtxY3O6xorKvE0L1q/YF+Hw9zoO8HgyoMa5jxuRvqxXunuwZtTWY89xUgoZoqMyzmChgNpRyIJcP+z3xanHaUlpTwN013oCvVFg5UUW5QYdKNL90w6+1WDpQ5pp0pT1KArBYTB8rsxJk1hrKyQqTDRHDYLnMgc6WRf+ucNaVCbReTAsTOEQPKviVKQtnH8i7lfDCg7IbEOzR7CpAOkwLEzlF2Q/YWVp0Dq4sITkfNBYaEOrvu14VW58GklWtcB+UyyzUuoz9ZOF1UKKk1OwAAAABJRU5ErkJggg==");
        assert_eq!(i.wiki_name.as_ref().unwrap(), "Helm of neitiznot");
        assert_eq!(
            i.wiki_url.as_ref().unwrap(),
            "https://oldschool.runescape.wiki/w/Helm_of_neitiznot"
        );
        assert_eq!(
            i.wiki_exchange.as_ref().unwrap(),
            "https://oldschool.runescape.wiki/w/Exchange:Helm_of_neitiznot"
        );

        let e = i.equipment.as_ref().unwrap();

        assert_eq!(e.attack_stab, 0);
        assert_eq!(e.attack_slash, 0);
        assert_eq!(e.attack_crush, 0);
        assert_eq!(e.attack_magic, 0);
        assert_eq!(e.attack_ranged, 0);

        assert_eq!(e.defence_stab, 31);
        assert_eq!(e.defence_slash, 29);
        assert_eq!(e.defence_crush, 34);
        assert_eq!(e.defence_magic, 3);
        assert_eq!(e.defence_ranged, 30);

        assert_eq!(e.melee_strength, 3);
        assert_eq!(e.ranged_strength, 0);
        assert_eq!(e.magic_damage, 0);
        assert_eq!(e.prayer, 3);

        assert_eq!(e.slot, Some(Slot::Head));
        assert_eq!(e.requirements.as_ref().unwrap().len(), 1);
        assert_eq!(e.requirements.as_ref().unwrap().get("defence"), Some(&55));

        assert!(i.weapon.is_none());

        // Projection
        let i = i.project().unwrap();

        assert_eq!(i.name, "Helm of neitiznot");
        assert_eq!(i.members, true);
        assert_eq!(i.weight, 2.267);
        assert_eq!(
            i.wiki_url,
            "https://oldschool.runescape.wiki/w/Helm_of_neitiznot"
        );

        assert_eq!(i.stats.attack.stab, 0);
        assert_eq!(i.stats.attack.slash, 0);
        assert_eq!(i.stats.attack.crush, 0);
        assert_eq!(i.stats.attack.magic, 0);
        assert_eq!(i.stats.attack.ranged, 0);

        assert_eq!(i.stats.defence.stab, 31);
        assert_eq!(i.stats.defence.slash, 29);
        assert_eq!(i.stats.defence.crush, 34);
        assert_eq!(i.stats.defence.magic, 3);
        assert_eq!(i.stats.defence.ranged, 30);

        assert_eq!(i.stats.melee_strength, 3);
        assert_eq!(i.stats.ranged_strength, 0);
        assert_eq!(i.stats.magic_damage, 0);
        assert_eq!(i.stats.prayer, 3);

        assert_eq!(i.slot, Slot::Head);

        assert_eq!(i.requirements.len(), 1);
        let r = &i.requirements[0];
        assert_eq!(r.skill, "defence");
        assert_eq!(r.level, 55);

        assert!(i.weapon.is_none());
    }

    #[test]
    fn test_weapon() {
        let i: ItemProperties = serde_json::from_str(
            r#"{"id": 4151, "name": "Abyssal whip", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 4152, "linked_id_placeholder": 14032, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": true, "cost": 120001, "lowalch": 48000, "highalch": 72000, "weight": 0.453, "buy_limit": 70, "quest_item": false, "release_date": "2005-01-26", "duplicate": false, "examine": "A weapon from the abyss.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAABvUlEQVR4Xu2Xv26DMBDG4QEyZECKIkVCKIoyVR26dOrQpUOHDn3/V3F7nL7689kGAo6z9JNuiH1wP+6PIU3zr2Jq3bRVkQ/Y7feBvfcn93o8upfDwT11XQKwOGQMwfYx9O7rcnaf52GEezuFgNdfn4JQ7RjAQojZLAAMcPJbAOH73PcloBTIQiEAG8MB7Pt6MQ+wSW1QAs6Kh1A/NgtnHyKMcZMUCP3AIBw0XcqmgU9qb4W0JwTIwuRAYHETF4FSIMkORjn1xCnjayy8lN/vLVY7TglfvBRGTJpZrpXM+my1f6DhPRdJs8M3nAPibGDseY9hVgHxzbh3chC22dkPQ8EnufddpBgI69Lk3B8hnPrwOsPEw7FYqUzoOsqBUtps8XUASh0bYbxZxUCcJZzCNnjOBGgDDBRD8d4tQAVgRAqEs2jusLOGEhaCgTQTgApLp/s5A0RBGMg3shx2YZb0fZUz9issD4VpsR4PkJ+uYbczJQr94rW7KT3yDJcegLtqeuRlAFa+0bdoeuTxPV2538Ixt1D86Vutr8IR16CSndzfoSpQLIDh09dmscL5lJICMUClw3JKD8tGXiVgfgACr1tEhnw7UAAAAABJRU5ErkJggg==", "wiki_name": "Abyssal whip", "wiki_url": "https://oldschool.runescape.wiki/w/Abyssal_whip", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Abyssal_whip", "equipment": {"attack_stab": 0, "attack_slash": 82, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 0, "defence_slash": 0, "defence_crush": 0, "defence_magic": 0, "defence_ranged": 0, "melee_strength": 82, "ranged_strength": 0, "magic_damage": 0, "prayer": 0, "slot": "weapon", "requirements": {"attack": 70}}, "weapon": {"attack_speed": 4, "weapon_type": "whips", "stances": [{"combat_style": "flick", "attack_type": "slash", "attack_style": "accurate", "experience": "attack", "boosts": null}, {"combat_style": "lash", "attack_type": "slash", "attack_style": "controlled", "experience": "shared", "boosts": null}, {"combat_style": "deflect", "attack_type": "slash", "attack_style": "defensive", "experience": "defence", "boosts": null}]}}"#,
        ).unwrap();

        assert_eq!(i.id, 4151);
        assert_eq!(i.name, "Abyssal whip");
        assert_eq!(i.incomplete, false);
        assert_eq!(i.members, true);
        assert_eq!(i.tradeable, true);
        assert_eq!(i.tradeable_on_ge, true);
        assert_eq!(i.stackable, false);
        assert!(i.stacked.is_none());
        assert_eq!(i.noted, false);
        assert_eq!(i.noteable, true);
        assert!(i.linked_id_item.is_none());
        assert_eq!(i.linked_id_noted, Some(4152));
        assert_eq!(i.linked_id_placeholder, Some(14032));
        assert_eq!(i.placeholder, false);
        assert_eq!(i.equipable, true);
        assert_eq!(i.equipable_by_player, true);
        assert_eq!(i.equipable_weapon, true);
        assert_eq!(i.cost, 120001);
        assert_eq!(i.lowalch, Some(48000));
        assert_eq!(i.highalch, Some(72000));
        assert_eq!(i.weight, Some(0.453));
        assert_eq!(i.buy_limit, Some(70));
        assert_eq!(i.quest_item, false);
        assert_eq!(i.release_date.as_ref().unwrap(), "2005-01-26");
        assert_eq!(i.duplicate, false);
        assert_eq!(i.examine.as_ref().unwrap(), "A weapon from the abyss.");
        assert_eq!(i.icon, "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAABvUlEQVR4Xu2Xv26DMBDG4QEyZECKIkVCKIoyVR26dOrQpUOHDn3/V3F7nL7689kGAo6z9JNuiH1wP+6PIU3zr2Jq3bRVkQ/Y7feBvfcn93o8upfDwT11XQKwOGQMwfYx9O7rcnaf52GEezuFgNdfn4JQ7RjAQojZLAAMcPJbAOH73PcloBTIQiEAG8MB7Pt6MQ+wSW1QAs6Kh1A/NgtnHyKMcZMUCP3AIBw0XcqmgU9qb4W0JwTIwuRAYHETF4FSIMkORjn1xCnjayy8lN/vLVY7TglfvBRGTJpZrpXM+my1f6DhPRdJs8M3nAPibGDseY9hVgHxzbh3chC22dkPQ8EnufddpBgI69Lk3B8hnPrwOsPEw7FYqUzoOsqBUtps8XUASh0bYbxZxUCcJZzCNnjOBGgDDBRD8d4tQAVgRAqEs2jusLOGEhaCgTQTgApLp/s5A0RBGMg3shx2YZb0fZUz9issD4VpsR4PkJ+uYbczJQr94rW7KT3yDJcegLtqeuRlAFa+0bdoeuTxPV2538Ixt1D86Vutr8IR16CSndzfoSpQLIDh09dmscL5lJICMUClw3JKD8tGXiVgfgACr1tEhnw7UAAAAABJRU5ErkJggg==");
        assert_eq!(i.wiki_name.as_ref().unwrap(), "Abyssal whip");
        assert_eq!(
            i.wiki_url.as_ref().unwrap(),
            "https://oldschool.runescape.wiki/w/Abyssal_whip"
        );
        assert_eq!(
            i.wiki_exchange.as_ref().unwrap(),
            "https://oldschool.runescape.wiki/w/Exchange:Abyssal_whip"
        );

        let e = i.equipment.as_ref().unwrap();

        assert_eq!(e.attack_stab, 0);
        assert_eq!(e.attack_slash, 82);
        assert_eq!(e.attack_crush, 0);
        assert_eq!(e.attack_magic, 0);
        assert_eq!(e.attack_ranged, 0);

        assert_eq!(e.defence_stab, 0);
        assert_eq!(e.defence_slash, 0);
        assert_eq!(e.defence_crush, 0);
        assert_eq!(e.defence_magic, 0);
        assert_eq!(e.defence_ranged, 0);

        assert_eq!(e.melee_strength, 82);
        assert_eq!(e.ranged_strength, 0);
        assert_eq!(e.magic_damage, 0);
        assert_eq!(e.prayer, 0);

        assert_eq!(e.slot, Some(Slot::Weapon));
        assert_eq!(e.requirements.as_ref().unwrap().len(), 1);
        assert_eq!(e.requirements.as_ref().unwrap().get("attack"), Some(&70));

        let w = i.weapon.as_ref().unwrap();

        assert_eq!(w.attack_speed, 4);
        assert_eq!(w.weapon_type, "whips");

        assert_eq!(w.stances.len(), 3);

        let s = &w.stances[0];
        assert_eq!(s.combat_style, "flick");
        assert_eq!(s.attack_type.as_ref().unwrap(), "slash");
        assert_eq!(s.attack_style.as_ref().unwrap(), "accurate");
        assert_eq!(s.experience.as_ref().unwrap(), "attack");
        assert!(s.boosts.is_none());

        let s = &w.stances[1];
        assert_eq!(s.combat_style, "lash");
        assert_eq!(s.attack_type.as_ref().unwrap(), "slash");
        assert_eq!(s.attack_style.as_ref().unwrap(), "controlled");
        assert_eq!(s.experience.as_ref().unwrap(), "shared");
        assert!(s.boosts.is_none());

        let s = &w.stances[2];
        assert_eq!(s.combat_style, "deflect");
        assert_eq!(s.attack_type.as_ref().unwrap(), "slash");
        assert_eq!(s.attack_style.as_ref().unwrap(), "defensive");
        assert_eq!(s.experience.as_ref().unwrap(), "defence");
        assert!(s.boosts.is_none());

        // Projection
        let i = i.project().unwrap();

        assert_eq!(i.name, "Abyssal whip");
        assert_eq!(i.members, true);
        assert_eq!(i.weight, 0.453);
        assert_eq!(
            i.wiki_url,
            "https://oldschool.runescape.wiki/w/Abyssal_whip"
        );

        assert_eq!(i.stats.attack.stab, 0);
        assert_eq!(i.stats.attack.slash, 82);
        assert_eq!(i.stats.attack.crush, 0);
        assert_eq!(i.stats.attack.magic, 0);
        assert_eq!(i.stats.attack.ranged, 0);

        assert_eq!(i.stats.defence.stab, 0);
        assert_eq!(i.stats.defence.slash, 0);
        assert_eq!(i.stats.defence.crush, 0);
        assert_eq!(i.stats.defence.magic, 0);
        assert_eq!(i.stats.defence.ranged, 0);

        assert_eq!(i.stats.melee_strength, 82);
        assert_eq!(i.stats.ranged_strength, 0);
        assert_eq!(i.stats.magic_damage, 0);
        assert_eq!(i.stats.prayer, 0);

        assert_eq!(i.slot, Slot::Weapon);

        assert_eq!(i.requirements.len(), 1);
        let r = &i.requirements[0];
        assert_eq!(r.skill, "attack");
        assert_eq!(r.level, 70);

        let w = i.weapon.as_ref().unwrap();
        assert_eq!(w.attack_delay, 4);
    }
}
