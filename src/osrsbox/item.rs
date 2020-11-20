mod stats;
mod weapon;

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

pub use stats::*;
pub use weapon::*;

pub(super) const PROJECTION_STRING: &str = r#"{"id":1,"name":1,"members":1,"weight":1,"wiki_url":1,"stats":1,"equipment":1,"weapon.attack_speed":1,"weapon.stances.combat_style":1,"weapon.stances.attack_type":1,"weapon.stances.attack_style":1}"#;

/// An equippable item.
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    #[serde_as(as = "DisplayFromStr")]
    id: usize,
    name: String,
    members: bool,
    weight: f32,
    wiki_url: String,
    #[serde(rename = "equipment")]
    stats: Stats,
    #[serde(rename = "weapon")]
    weapon_data: Option<WeaponData>,
}

impl Item {
    /// Id of the item.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Name of the item.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true for members-only items.
    pub fn members(&self) -> bool {
        self.members
    }

    /// Weight of the item.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Link to the OSRS Wiki.
    pub fn wiki_url(&self) -> &str {
        &self.wiki_url
    }

    /// Reference to the stats of the item.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Get equipment slot.
    pub fn slot(&self) -> Slot {
        self.stats.slot()
    }

    /// Weapon-specific data. Returns [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) for non-weapons.
    pub fn weapon_data(&self) -> Option<&WeaponData> {
        self.weapon_data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn deserialize_weapon() -> Result<()> {
        let i: Item = serde_json::from_str(
            r#"{"_id": "5fa6f310c9559b51a90138c7", "id": "4151", "name": "Abyssal whip", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 4152, "linked_id_placeholder": 14032, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": true, "cost": 120001, "lowalch": 48000, "highalch": 72000, "weight": 0.453, "buy_limit": 70, "quest_item": false, "release_date": "2005-01-26", "duplicate": false, "examine": "A weapon from the abyss.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAABvUlEQVR4Xu2Xv26DMBDG4QEyZECKIkVCKIoyVR26dOrQpUOHDn3/V3F7nL7689kGAo6z9JNuiH1wP+6PIU3zr2Jq3bRVkQ/Y7feBvfcn93o8upfDwT11XQKwOGQMwfYx9O7rcnaf52GEezuFgNdfn4JQ7RjAQojZLAAMcPJbAOH73PcloBTIQiEAG8MB7Pt6MQ+wSW1QAs6Kh1A/NgtnHyKMcZMUCP3AIBw0XcqmgU9qb4W0JwTIwuRAYHETF4FSIMkORjn1xCnjayy8lN/vLVY7TglfvBRGTJpZrpXM+my1f6DhPRdJs8M3nAPibGDseY9hVgHxzbh3chC22dkPQ8EnufddpBgI69Lk3B8hnPrwOsPEw7FYqUzoOsqBUtps8XUASh0bYbxZxUCcJZzCNnjOBGgDDBRD8d4tQAVgRAqEs2jusLOGEhaCgTQTgApLp/s5A0RBGMg3shx2YZb0fZUz9issD4VpsR4PkJ+uYbczJQr94rW7KT3yDJcegLtqeuRlAFa+0bdoeuTxPV2538Ixt1D86Vutr8IR16CSndzfoSpQLIDh09dmscL5lJICMUClw3JKD8tGXiVgfgACr1tEhnw7UAAAAABJRU5ErkJggg==", "wiki_name": "Abyssal whip", "wiki_url": "https://oldschool.runescape.wiki/w/Abyssal_whip", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Abyssal_whip", "equipment": {"attack_stab": 0, "attack_slash": 82, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 0, "defence_slash": 0, "defence_crush": 0, "defence_magic": 0, "defence_ranged": 0, "melee_strength": 82, "ranged_strength": 0, "magic_damage": 0, "prayer": 0, "slot": "weapon", "requirements": {"attack": 70}}, "weapon": {"attack_speed": 4, "weapon_type": "whips", "stances": [{"combat_style": "flick", "attack_type": "slash", "attack_style": "accurate", "experience": "attack", "boosts": null}, {"combat_style": "lash", "attack_type": "slash", "attack_style": "controlled", "experience": "shared", "boosts": null}, {"combat_style": "deflect", "attack_type": "slash", "attack_style": "defensive", "experience": "defence", "boosts": null}]}, "_created": "Thu, 01 Jan 1970 00:00:00 GMT", "_updated": "Thu, 01 Jan 1970 00:00:00 GMT", "_etag": "8c9f7a34a0e869651420f4201b176ac65505d23a", "_links": {"self": {"title": "Item", "href": "items/5fa6f310c9559b51a90138c7"}}}"#,
        )?;

        assert_eq!(i.id(), 4151);
        assert_eq!(i.name(), "Abyssal whip");
        assert_eq!(i.members(), true);
        assert_eq!(i.weight(), 0.453);
        assert_eq!(i.slot(), Slot::Weapon);
        assert_eq!(
            i.wiki_url(),
            "https://oldschool.runescape.wiki/w/Abyssal_whip"
        );

        assert_eq!(i.stats()[Stat::Attack(AttackType::Stab)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Slash)], 82);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Crush)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Magic)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Ranged)], 0);

        assert_eq!(i.stats()[Stat::Defence(AttackType::Stab)], 0);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Slash)], 0);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Crush)], 0);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Magic)], 0);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Ranged)], 0);

        assert_eq!(i.stats()[Stat::MeleeStrength], 82);
        assert_eq!(i.stats()[Stat::RangedStrength], 0);
        assert_eq!(i.stats()[Stat::MagicDamage], 0);
        assert_eq!(i.stats()[Stat::Prayer], 0);

        assert_eq!(i.stats().attack().avg(), 16.4);
        assert_eq!(i.stats().attack().median(), 0);
        assert_eq!(i.stats().attack().melee_avg(), 27.0 + 1.0 / 3.0);
        assert_eq!(i.stats().defence().avg(), 0.0);
        assert_eq!(i.stats().defence().median(), 0);
        assert_eq!(i.stats().defence().melee_avg(), 0.0);

        let w = i.weapon_data().unwrap();
        assert_eq!(w.attack_delay(), 4);

        {
            let s = &w.stances()[0];
            assert_eq!(s.name(), "Flick");
            assert_eq!(s.attack_type(), Some(AttackType::Slash));
            assert_eq!(s.attack_style(), Some(AttackStyle::Accurate));
        }

        {
            let s = &w.stances()[1];
            assert_eq!(s.name(), "Lash");
            assert_eq!(s.attack_type(), Some(AttackType::Slash));
            assert_eq!(s.attack_style(), Some(AttackStyle::Controlled));
        }

        {
            let s = &w.stances()[2];
            assert_eq!(s.name(), "Deflect");
            assert_eq!(s.attack_type(), Some(AttackType::Slash));
            assert_eq!(s.attack_style(), Some(AttackStyle::Defensive));
        }

        Ok(())
    }

    #[test]
    fn deserialize_equipment() -> Result<()> {
        let i: Item = serde_json::from_str(
            r#"{"_id": "5fa6f312c9559b51a90150e2", "id": "10828", "name": "Helm of neitiznot", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 10843, "linked_id_placeholder": 17668, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": false, "cost": 50000, "lowalch": 20000, "highalch": 30000, "weight": 2.267, "buy_limit": 70, "quest_item": false, "release_date": "2007-02-06", "duplicate": false, "examine": "A gift from Neitiznot's Burgher.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAACyElEQVR4XtWW70tTURzG/f4JvghEEMYQkbEXIxziaDgUh4aKhflCLeb6IXO5tDWtpRnq1PzBpm2L/NVaSkkjUQQtpfBVRC+iv+jJcy67zOt058TuogcujJ0v5/mc55z7Pbeo6P8WQfvPxZKtlxIhHApB3IQQn5+TqJcWYXtjgxtYDAYBE8K7V3GJeikRYrMv1MlzGyhpbieTegAR1paW1O0Sm5ywEolILEBKhPnxNonJCcvhMD6srkgsQFiEZKybb1d6xdqKs1K2d2FiQh+g6dFmDPXXI5VICBgQUus9mB1rlUhUSoTxx03oc7sFDQivFzoQmZoSgP8rEUb8TrycmRE0IISCV3miYvVSUuIf8HjwJhoVNCCMBRoF05QWYX2xEw0OB7bW14SBngw0CNZK66S5TV6D3+vVrJi9adpHqd9adaGns1M/oMz40+Zs+9iZmgwGOezt7i51LDrTroHPq5QDmjZj9xhrdm9jsVNALJFyo5HXnF5A3kVqPznc2cF+KoWdzU28X77FG19o5CkC/fd5Qs1OJy4VF+t5fpgIi1PX8XVvD0e7uxzq80cPPyfss2L62SiGH/hUILPJhEfeOj2BmAjHB/scigEdpvrwKeE+k1BrUyOsFgt8d+3/BigzoV6XC+0tLai12U5+2/QH+vUtgJ+Hfvz48lAFSkS7+Haya2WwtxY3O6xorKvE0L1q/YF+Hw9zoO8HgyoMa5jxuRvqxXunuwZtTWY89xUgoZoqMyzmChgNpRyIJcP+z3xanHaUlpTwN013oCvVFg5UUW5QYdKNL90w6+1WDpQ5pp0pT1KArBYTB8rsxJk1hrKyQqTDRHDYLnMgc6WRf+ucNaVCbReTAsTOEQPKviVKQtnH8i7lfDCg7IbEOzR7CpAOkwLEzlF2Q/YWVp0Dq4sITkfNBYaEOrvu14VW58GklWtcB+UyyzUuoz9ZOF1UKKk1OwAAAABJRU5ErkJggg==", "wiki_name": "Helm of neitiznot", "wiki_url": "https://oldschool.runescape.wiki/w/Helm_of_neitiznot", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Helm_of_neitiznot", "equipment": {"attack_stab": 0, "attack_slash": 0, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 31, "defence_slash": 29, "defence_crush": 34, "defence_magic": 3, "defence_ranged": 30, "melee_strength": 3, "ranged_strength": 0, "magic_damage": 0, "prayer": 3, "slot": "head", "requirements": {"defence": 55}}, "weapon": null, "_created": "Thu, 01 Jan 1970 00:00:00 GMT", "_updated": "Thu, 01 Jan 1970 00:00:00 GMT", "_etag": "809d8056ae8cd67356b92d96c8d444eb24e00554", "_links": {"self": {"title": "Item", "href": "items/5fa6f312c9559b51a90150e2"}}}"#,
        )?;

        assert_eq!(i.id(), 10828);
        assert_eq!(i.name(), "Helm of neitiznot");
        assert_eq!(i.members(), true);
        assert_eq!(i.weight(), 2.267);
        assert_eq!(i.slot(), Slot::Head);
        assert_eq!(
            i.wiki_url(),
            "https://oldschool.runescape.wiki/w/Helm_of_neitiznot"
        );

        assert_eq!(i.stats()[Stat::Attack(AttackType::Stab)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Slash)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Crush)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Magic)], 0);
        assert_eq!(i.stats()[Stat::Attack(AttackType::Ranged)], 0);

        assert_eq!(i.stats()[Stat::Defence(AttackType::Stab)], 31);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Slash)], 29);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Crush)], 34);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Magic)], 3);
        assert_eq!(i.stats()[Stat::Defence(AttackType::Ranged)], 30);

        assert_eq!(i.stats()[Stat::MeleeStrength], 3);
        assert_eq!(i.stats()[Stat::RangedStrength], 0);
        assert_eq!(i.stats()[Stat::MagicDamage], 0);
        assert_eq!(i.stats()[Stat::Prayer], 3);

        assert_eq!(i.stats().attack().avg(), 0.0);
        assert_eq!(i.stats().attack().median(), 0);
        assert_eq!(i.stats().attack().melee_avg(), 0.0);
        assert_eq!(i.stats().defence().avg(), 25.4);
        assert_eq!(i.stats().defence().median(), 30);
        assert_eq!(i.stats().defence().melee_avg(), 31.0 + 1.0 / 3.0);

        assert!(i.weapon_data().is_none());

        Ok(())
    }
}
