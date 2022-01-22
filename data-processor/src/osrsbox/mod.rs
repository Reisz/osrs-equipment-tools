//! Data format used by the [OSRSBox](https://www.osrsbox.com/) API.

pub use item_equipment::*;
pub use item_properties::*;
pub use item_weapon::*;

mod item_equipment;
mod item_properties;
mod item_weapon;

#[cfg(test)]
mod tests {
    use data::{DamageType, EquipSlot, Skill};
    use enum_iterator::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_equipment() {
        let i: ItemProperties = serde_json::from_str(
            r#"{"id": 10828, "name": "Helm of neitiznot", "last_updated": "2021-08-05", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 10843, "linked_id_placeholder": 17668, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": false, "cost": 50000, "lowalch": 20000, "highalch": 30000, "weight": 2.267, "buy_limit": 70, "quest_item": false, "release_date": "2007-02-06", "duplicate": false, "examine": "A gift from Neitiznot's Burgher.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAACyElEQVR4XtWW70tTURzG/f4JvghEEMYQkbEXIxziaDgUh4aKhflCLeb6IXO5tDWtpRnq1PzBpm2L/NVaSkkjUQQtpfBVRC+iv+jJcy67zOt058TuogcujJ0v5/mc55z7Pbeo6P8WQfvPxZKtlxIhHApB3IQQn5+TqJcWYXtjgxtYDAYBE8K7V3GJeikRYrMv1MlzGyhpbieTegAR1paW1O0Sm5ywEolILEBKhPnxNonJCcvhMD6srkgsQFiEZKybb1d6xdqKs1K2d2FiQh+g6dFmDPXXI5VICBgQUus9mB1rlUhUSoTxx03oc7sFDQivFzoQmZoSgP8rEUb8TrycmRE0IISCV3miYvVSUuIf8HjwJhoVNCCMBRoF05QWYX2xEw0OB7bW14SBngw0CNZK66S5TV6D3+vVrJi9adpHqd9adaGns1M/oMz40+Zs+9iZmgwGOezt7i51LDrTroHPq5QDmjZj9xhrdm9jsVNALJFyo5HXnF5A3kVqPznc2cF+KoWdzU28X77FG19o5CkC/fd5Qs1OJy4VF+t5fpgIi1PX8XVvD0e7uxzq80cPPyfss2L62SiGH/hUILPJhEfeOj2BmAjHB/scigEdpvrwKeE+k1BrUyOsFgt8d+3/BigzoV6XC+0tLai12U5+2/QH+vUtgJ+Hfvz48lAFSkS7+Haya2WwtxY3O6xorKvE0L1q/YF+Hw9zoO8HgyoMa5jxuRvqxXunuwZtTWY89xUgoZoqMyzmChgNpRyIJcP+z3xanHaUlpTwN013oCvVFg5UUW5QYdKNL90w6+1WDpQ5pp0pT1KArBYTB8rsxJk1hrKyQqTDRHDYLnMgc6WRf+ucNaVCbReTAsTOEQPKviVKQtnH8i7lfDCg7IbEOzR7CpAOkwLEzlF2Q/YWVp0Dq4sITkfNBYaEOrvu14VW58GklWtcB+UyyzUuoz9ZOF1UKKk1OwAAAABJRU5ErkJggg==", "wiki_name": "Helm of neitiznot", "wiki_url": "https://oldschool.runescape.wiki/w/Helm_of_neitiznot", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Helm_of_neitiznot", "equipment": {"attack_stab": 0, "attack_slash": 0, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 31, "defence_slash": 29, "defence_crush": 34, "defence_magic": 3, "defence_ranged": 30, "melee_strength": 3, "ranged_strength": 0, "magic_damage": 0, "prayer": 3, "slot": "head", "requirements": {"defence": 55}}, "weapon": null}"#,
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

        assert_eq!(e.slot, EquipSlot::Head);
        assert_eq!(e.requirements.as_ref().unwrap().len(), 1);
        assert_eq!(
            e.requirements.as_ref().unwrap().get(&Skill::Defence),
            Some(&55)
        );

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

        for (damage_type, (att, def)) in
            DamageType::into_enum_iter().zip([(0, 31), (0, 29), (0, 34), (0, 3), (0, 30)])
        {
            assert_eq!(i.combat_stats.attack[damage_type], att);
            assert_eq!(i.combat_stats.defence[damage_type], def);
        }

        assert_eq!(i.combat_stats.melee_strength, 3);
        assert_eq!(i.combat_stats.ranged_strength, 0);
        assert_eq!(i.combat_stats.magic_damage, 0);
        assert_eq!(i.combat_stats.prayer, 3);

        assert_eq!(i.equip_slot, EquipSlot::Head);

        assert_eq!(i.requirements.len(), 1);
        let r = &i.requirements[0];
        assert_eq!(r.skill, Skill::Defence);
        assert_eq!(r.level, 55);

        assert!(i.weapon_data.is_none());
    }

    #[test]
    fn test_weapon() {
        let i: ItemProperties = serde_json::from_str(
            r#"{"id": 4151, "name": "Abyssal whip", "last_updated": "2021-08-05", "incomplete": false, "members": true, "tradeable": true, "tradeable_on_ge": true, "stackable": false, "stacked": null, "noted": false, "noteable": true, "linked_id_item": null, "linked_id_noted": 4152, "linked_id_placeholder": 14032, "placeholder": false, "equipable": true, "equipable_by_player": true, "equipable_weapon": true, "cost": 120001, "lowalch": 48000, "highalch": 72000, "weight": 0.453, "buy_limit": 70, "quest_item": false, "release_date": "2005-01-26", "duplicate": false, "examine": "A weapon from the abyss.", "icon": "iVBORw0KGgoAAAANSUhEUgAAACQAAAAgCAYAAAB6kdqOAAABvUlEQVR4Xu2Xv26DMBDG4QEyZECKIkVCKIoyVR26dOrQpUOHDn3/V3F7nL7689kGAo6z9JNuiH1wP+6PIU3zr2Jq3bRVkQ/Y7feBvfcn93o8upfDwT11XQKwOGQMwfYx9O7rcnaf52GEezuFgNdfn4JQ7RjAQojZLAAMcPJbAOH73PcloBTIQiEAG8MB7Pt6MQ+wSW1QAs6Kh1A/NgtnHyKMcZMUCP3AIBw0XcqmgU9qb4W0JwTIwuRAYHETF4FSIMkORjn1xCnjayy8lN/vLVY7TglfvBRGTJpZrpXM+my1f6DhPRdJs8M3nAPibGDseY9hVgHxzbh3chC22dkPQ8EnufddpBgI69Lk3B8hnPrwOsPEw7FYqUzoOsqBUtps8XUASh0bYbxZxUCcJZzCNnjOBGgDDBRD8d4tQAVgRAqEs2jusLOGEhaCgTQTgApLp/s5A0RBGMg3shx2YZb0fZUz9issD4VpsR4PkJ+uYbczJQr94rW7KT3yDJcegLtqeuRlAFa+0bdoeuTxPV2538Ixt1D86Vutr8IR16CSndzfoSpQLIDh09dmscL5lJICMUClw3JKD8tGXiVgfgACr1tEhnw7UAAAAABJRU5ErkJggg==", "wiki_name": "Abyssal whip", "wiki_url": "https://oldschool.runescape.wiki/w/Abyssal_whip", "wiki_exchange": "https://oldschool.runescape.wiki/w/Exchange:Abyssal_whip", "equipment": {"attack_stab": 0, "attack_slash": 82, "attack_crush": 0, "attack_magic": 0, "attack_ranged": 0, "defence_stab": 0, "defence_slash": 0, "defence_crush": 0, "defence_magic": 0, "defence_ranged": 0, "melee_strength": 82, "ranged_strength": 0, "magic_damage": 0, "prayer": 0, "slot": "weapon", "requirements": {"attack": 70}}, "weapon": {"attack_speed": 4, "weapon_type": "whips", "stances": [{"combat_style": "flick", "attack_type": "slash", "attack_style": "accurate", "experience": "attack", "boosts": null}, {"combat_style": "lash", "attack_type": "slash", "attack_style": "controlled", "experience": "shared", "boosts": null}, {"combat_style": "deflect", "attack_type": "slash", "attack_style": "defensive", "experience": "defence", "boosts": null}]}}"#,
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

        assert_eq!(e.slot, EquipSlot::Weapon);
        assert_eq!(e.requirements.as_ref().unwrap().len(), 1);
        assert_eq!(
            e.requirements.as_ref().unwrap().get(&Skill::Attack),
            Some(&70)
        );

        let w = i.weapon.as_ref().unwrap();

        assert_eq!(w.attack_speed, 4);
        assert_eq!(w.weapon_type, "whips");

        assert_eq!(w.stances.len(), 3);

        let s = &w.stances[0];
        assert_eq!(s.combat_style, "flick");
        assert_eq!(s.attack_type, Some(OsrsboxAttackType::Slash));
        assert_eq!(s.attack_style, Some(OsrsboxAttackStyle::Accurate));
        assert_eq!(s.experience.as_ref().unwrap(), "attack");
        assert!(s.boosts.is_none());

        let s = &w.stances[1];
        assert_eq!(s.combat_style, "lash");
        assert_eq!(s.attack_type, Some(OsrsboxAttackType::Slash));
        assert_eq!(s.attack_style, Some(OsrsboxAttackStyle::Controlled));
        assert_eq!(s.experience.as_ref().unwrap(), "shared");
        assert!(s.boosts.is_none());

        let s = &w.stances[2];
        assert_eq!(s.combat_style, "deflect");
        assert_eq!(s.attack_type, Some(OsrsboxAttackType::Slash));
        assert_eq!(s.attack_style, Some(OsrsboxAttackStyle::Defensive));
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

        for (damage_type, (att, def)) in
            DamageType::into_enum_iter().zip([(0, 0), (82, 0), (0, 0), (0, 0), (0, 0)])
        {
            assert_eq!(i.combat_stats.attack[damage_type], att);
            assert_eq!(i.combat_stats.defence[damage_type], def);
        }

        assert_eq!(i.combat_stats.melee_strength, 82);
        assert_eq!(i.combat_stats.ranged_strength, 0);
        assert_eq!(i.combat_stats.magic_damage, 0);
        assert_eq!(i.combat_stats.prayer, 0);

        assert_eq!(i.equip_slot, EquipSlot::Weapon);

        assert_eq!(i.requirements.len(), 1);
        let r = &i.requirements[0];
        assert_eq!(r.skill, Skill::Attack);
        assert_eq!(r.level, 70);

        assert_eq!(i.weapon_data.unwrap().attack_delay, 4);
    }
}
