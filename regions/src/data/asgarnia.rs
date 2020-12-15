//! Items mainly locked by Asgarnia.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Asgarnia)
}

/// Add requirements for the following items:
/// - [Boot upgrades](https://oldschool.runescape.wiki/w/Cerberus)
/// - [God Wars Dungeon](https://oldschool.runescape.wiki/w/God_Wars_Dungeon)
/// - [Defenders](https://oldschool.runescape.wiki/w/Defender)
/// - [Initiate armour](https://oldschool.runescape.wiki/w/Initiate_armour)
/// - [Void](https://oldschool.runescape.wiki/w/Void_Knight_equipment)
/// - [Falador shield](https://oldschool.runescape.wiki/w/Falador_shield)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Falador shield 1".to_string(), expr());
    map.insert("Falador shield 2".to_string(), expr());
    map.insert("Falador shield 3".to_string(), expr());
    map.insert("Falador shield 4".to_string(), expr());

    add_cerb(map);
    add_gwd(map);
    add_defenders(map);
    add_initiate(map);
    add_void(map);
}

fn add_cerb(map: &mut ExprMap) {
    map.insert("Primordial boots".to_string(), expr());
    map.insert("Pegasian boots".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Desert);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();

    map.insert("Eternal boots".to_string(), expr);
}

fn add_gwd(map: &mut ExprMap) {
    map.insert("Dragon boots".to_string(), expr());

    map.insert("Armadyl helmet".to_string(), expr());
    map.insert("Armadyl chestplate".to_string(), expr());
    map.insert("Armadyl chainskirt".to_string(), expr());
    map.insert("Armadyl godsword".to_string(), expr());

    map.insert("Bandos chestplate".to_string(), expr());
    map.insert("Bandos tassets".to_string(), expr());
    map.insert("Bandos boots".to_string(), expr());
    map.insert("Bandos godsword".to_string(), expr());

    map.insert("Saradomin sword".to_string(), expr());
    map.insert("Armadyl crossbow".to_string(), expr());
    map.insert("Saradomin godsword".to_string(), expr());

    map.insert("Steam battlestaff".to_string(), expr());
    map.insert("Zamorakian spear".to_string(), expr());
    map.insert("Zamorak godsword".to_string(), expr());
}

fn add_defenders(map: &mut ExprMap) {
    map.insert("Bronze defender (Normal)".to_string(), expr());
    map.insert("Iron defender (Normal)".to_string(), expr());
    map.insert("Steel defender (Normal)".to_string(), expr());
    map.insert("Black defender (Normal)".to_string(), expr());
    map.insert("Mithril defender (Normal)".to_string(), expr());
    map.insert("Adamant defender (Normal)".to_string(), expr());
    map.insert("Rune defender (Normal)".to_string(), expr());
    map.insert("Dragon defender (Normal)".to_string(), expr());
}

fn add_initiate(map: &mut ExprMap) {
    map.insert("Initiate sallet".to_string(), expr());
    map.insert("Initiate hauberk".to_string(), expr());
    map.insert("Initiate cuisse".to_string(), expr());
}

fn add_void(map: &mut ExprMap) {
    map.insert("Void melee helm (Normal)".to_string(), expr());
    map.insert("Void mage helm (Normal)".to_string(), expr());
    map.insert("Void ranger helm (Normal)".to_string(), expr());

    map.insert("Void knight gloves (Normal)".to_string(), expr());
    map.insert("Void knight robe (Normal)".to_string(), expr());
    map.insert("Void knight top (Normal)".to_string(), expr());

    map.insert("Void knight mace (Normal)".to_string(), expr());
    map.insert("Void seal (8)".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Asgarnia);
    expr.var(Region::Kandarin);
    expr.var(Region::Tirannwn);
    expr.and();
    expr.and();
    let expr = expr.finalize().unwrap();

    map.insert("Elite void robe (Normal)".to_string(), expr.clone());
    map.insert("Elite void top (Normal)".to_string(), expr);
}
