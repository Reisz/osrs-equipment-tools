//! Items mainly locked by Fremennik.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Fremennik)
}

/// Add requirements for the following items:
/// - [Dragon Slayer II](https://oldschool.runescape.wiki/w/Dragon_Slayer_II#Rewards)
/// - [Dagannoth Kings](https://oldschool.runescape.wiki/w/Dagannoth_Kings)
/// - [Fremennik Quests](https://oldschool.runescape.wiki/w/Quests/Series#Fremennik_series)
pub fn add_items(map: &mut ExprMap) {
    add_ds2(map);
    add_dks(map);
    add_quests(map);
}

fn add_ds2(map: &mut ExprMap) {
    map.insert("Dragonfire ward".to_string(), expr());
    map.insert("Dragonbone necklace".to_string(), expr());
    map.insert("Ava's assembler".to_string(), expr());
    map.insert("Dragon crossbow".to_string(), expr());
    map.insert("Dragon kiteshield".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Desert);
    expr.var(Region::Kandarin);
    expr.or();
    expr.var(Region::Fremennik);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Dragon platebody".to_string(), expr);

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Kandarin);
    expr.var(Region::Fremennik);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Mythical cape".to_string(), expr);
}

fn add_dks(map: &mut ExprMap) {
    map.insert("Dragon axe".to_string(), expr());

    map.insert("Mud battlestaff".to_string(), expr());
    map.insert("Seers ring".to_string(), expr());

    map.insert("Berserker ring".to_string(), expr());
    map.insert("Warrior ring".to_string(), expr());

    map.insert("Seercull".to_string(), expr());
    map.insert("Archers ring".to_string(), expr());
}

fn add_quests(map: &mut ExprMap) {
    // https://oldschool.runescape.wiki/w/The_Fremennik_Trials
    map.insert("Fremennik helm".to_string(), expr());
    // TODO: These are equivalent to regular metal items and should be filtered
    map.insert("Fremennik shield".to_string(), expr());
    map.insert("Fremennik blade".to_string(), expr());

    map.insert("Warrior helm".to_string(), expr());
    map.insert("Berserker helm".to_string(), expr());
    map.insert("Archer helm".to_string(), expr());
    map.insert("Farseer helm".to_string(), expr());

    // https://oldschool.runescape.wiki/w/Rock-shell_armour
    map.insert("Rock-shell helm".to_string(), expr());
    map.insert("Rock-shell plate".to_string(), expr());
    map.insert("Rock-shell legs".to_string(), expr());
    map.insert("Rock-shell gloves".to_string(), expr());
    map.insert("Rock-shell boots".to_string(), expr());

    // https://oldschool.runescape.wiki/w/Skeletal_armour
    map.insert("Skeletal helm".to_string(), expr());
    map.insert("Skeletal top".to_string(), expr());
    map.insert("Skeletal bottoms".to_string(), expr());
    map.insert("Skeletal gloves".to_string(), expr());
    map.insert("Skeletal boots".to_string(), expr());

    // https://oldschool.runescape.wiki/w/Spined_armour
    map.insert("Spined helm".to_string(), expr());
    map.insert("Spined body".to_string(), expr());
    map.insert("Spined chaps".to_string(), expr());
    map.insert("Spined gloves".to_string(), expr());
    map.insert("Spined boots".to_string(), expr());

    // https://oldschool.runescape.wiki/w/God_book
    map.insert("Book of balance".to_string(), expr());
    map.insert("Book of darkness".to_string(), expr());
    map.insert("Book of law".to_string(), expr());
    map.insert("Book of war".to_string(), expr());
    map.insert("Holy book".to_string(), expr());
    map.insert("Unholy book".to_string(), expr());

    // https://oldschool.runescape.wiki/w/Lunar_equipment
    map.insert("Lunar boots".to_string(), expr());
    map.insert("Lunar cape".to_string(), expr());
    map.insert("Lunar gloves".to_string(), expr());
    map.insert("Lunar helm".to_string(), expr());
    map.insert("Lunar legs".to_string(), expr());
    map.insert("Lunar torso".to_string(), expr());
    map.insert("Lunar amulet".to_string(), expr());
    map.insert("Lunar ring".to_string(), expr());
    map.insert("Lunar staff".to_string(), expr());

    // https://oldschool.runescape.wiki/w/Moonclan_robes
    map.insert("Moonclan helm".to_string(), expr());
    map.insert("Moonclan hat".to_string(), expr());
    map.insert("Moonclan armour".to_string(), expr());
    map.insert("Moonclan skirt".to_string(), expr());
    map.insert("Moonclan gloves".to_string(), expr());
    map.insert("Moonclan boots".to_string(), expr());
    map.insert("Moonclan cape".to_string(), expr());

    // https://oldschool.runescape.wiki/w/The_Fremennik_Isles#Rewards
    map.insert("Helm of neitiznot".to_string(), expr());

    map.insert("Silly jester boots".to_string(), expr());
    map.insert("Silly jester hat".to_string(), expr());
    map.insert("Silly jester tights".to_string(), expr());
    map.insert("Silly jester top".to_string(), expr());

    map.insert("Neitiznot shield".to_string(), expr());
    // TODO Yak-hide (name-collision)

    // https://oldschool.runescape.wiki/w/The_Fremennik_Exiles#Rewards
    map.insert("V's shield".to_string(), expr());
    map.insert("Neitiznot faceguard".to_string(), expr());
}
