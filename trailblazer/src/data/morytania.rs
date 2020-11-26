//! Items mainly locked by Morytania.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Morytania)
}

/// Add requirements for the following items:
/// - [Barrows equipment](https://oldschool.runescape.wiki/w/Barrows_equipment)
/// - [Grotesque Guardians](https://oldschool.runescape.wiki/w/Grotesque_Guardians) / Gargoyles
///     - The super-rare [Bandos boots](https://oldschool.runescape.wiki/w/Bandos_boots) drop
///       from elite clue steps is not counted here.
/// - [Theatre of Blood](https://oldschool.runescape.wiki/w/Theatre_of_Blood#Loot_table)
/// - [The Nightmare](https://oldschool.runescape.wiki/w/The_Nightmare)
/// - Other
///     - [Amulet of blood fury](https://oldschool.runescape.wiki/w/Amulet_of_blood_fury)
///     - [Black mask](https://oldschool.runescape.wiki/w/Black_mask)
pub fn add_items(map: &mut ExprMap) {
    add_barrows(map);
    add_guardians(map);
    add_tob(map);
    add_nightmare(map);
    add_other(map);
}

fn add_barrows(map: &mut ExprMap) {
    map.insert("Ahrim's hood".to_string(), expr());
    map.insert("Ahrim's robeskirt".to_string(), expr());
    map.insert("Ahrim's robetop".to_string(), expr());
    map.insert("Ahrim's staff".to_string(), expr());

    map.insert("Dharok's greataxe".to_string(), expr());
    map.insert("Dharok's helm".to_string(), expr());
    map.insert("Dharok's platebody".to_string(), expr());
    map.insert("Dharok's platelegs".to_string(), expr());

    map.insert("Guthan's chainskirt".to_string(), expr());
    map.insert("Guthan's helm".to_string(), expr());
    map.insert("Guthan's platebody".to_string(), expr());
    map.insert("Guthan's warspear".to_string(), expr());

    map.insert("Karil's coif".to_string(), expr());
    map.insert("Karil's crossbow".to_string(), expr());
    map.insert("Karil's leatherskirt".to_string(), expr());
    map.insert("Karil's leathertop".to_string(), expr());

    map.insert("Torag's hammers".to_string(), expr());
    map.insert("Torag's helm".to_string(), expr());
    map.insert("Torag's platebody".to_string(), expr());
    map.insert("Torag's platelegs".to_string(), expr());

    map.insert("Verac's brassard".to_string(), expr());
    map.insert("Verac's flail".to_string(), expr());
    map.insert("Verac's helm".to_string(), expr());
    map.insert("Verac's plateskirt".to_string(), expr());
}

fn add_guardians(map: &mut ExprMap) {
    map.insert("Granite maul".to_string(), expr());
    map.insert("Granite gloves".to_string(), expr());
    map.insert("Granite ring".to_string(), expr());
    map.insert("Granite hammer".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Morytania);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Guardian boots".to_string(), expr);
}

fn add_tob(map: &mut ExprMap) {
    map.insert("Ghrazi rapier".to_string(), expr());
    map.insert("Sanguinesti staff".to_string(), expr());
    map.insert("Scythe of vitur".to_string(), expr());

    map.insert("Justiciar faceguard".to_string(), expr());
    map.insert("Justiciar chestguard".to_string(), expr());
    map.insert("Justiciar legguards".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Morytania);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Avernic defender".to_string(), expr);
}

fn add_nightmare(map: &mut ExprMap) {
    map.insert("Nightmare staff".to_string(), expr());

    map.insert("Inquisitor's great helm".to_string(), expr());
    map.insert("Inquisitor's hauberk".to_string(), expr());
    map.insert("Inquisitor's plateskirt".to_string(), expr());
    map.insert("Inquisitor's mace".to_string(), expr());
}

fn add_other(map: &mut ExprMap) {
    map.insert("Black mask".to_string(), expr());
    map.insert("Slayer helmet".to_string(), expr());
    map.insert("Slayer helmet (i)".to_string(), expr());
    map.insert("Amulet of blood fury".to_string(), expr());
}
