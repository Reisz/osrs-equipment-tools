//! Items mainly locked by Morytania.

use super::ExprMap;
use crate::{bool_expr::BoolExpr, vars::Region};

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
/// - [Morytania legs](https://oldschool.runescape.wiki/w/Morytania_legs)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Morytania legs 1".to_string(), expr());
    map.insert("Morytania legs 2".to_string(), expr());
    map.insert("Morytania legs 3".to_string(), expr());
    map.insert("Morytania legs 4".to_string(), expr());

    add_barrows(map);
    add_guardians(map);
    add_tob(map);
    add_nightmare(map);
    add_other(map);
}

fn add_barrows(map: &mut ExprMap) {
    map.insert("Ahrim's hood (Undamaged)".to_string(), expr());
    map.insert("Ahrim's robeskirt (Undamaged)".to_string(), expr());
    map.insert("Ahrim's robetop (Undamaged)".to_string(), expr());
    map.insert("Ahrim's staff (Undamaged)".to_string(), expr());

    map.insert("Dharok's greataxe (Undamaged)".to_string(), expr());
    map.insert("Dharok's helm (Undamaged)".to_string(), expr());
    map.insert("Dharok's platebody (Undamaged)".to_string(), expr());
    map.insert("Dharok's platelegs (Undamaged)".to_string(), expr());

    map.insert("Guthan's chainskirt (Undamaged)".to_string(), expr());
    map.insert("Guthan's helm (Undamaged)".to_string(), expr());
    map.insert("Guthan's platebody (Undamaged)".to_string(), expr());
    map.insert("Guthan's warspear (Undamaged)".to_string(), expr());

    map.insert("Karil's coif (Undamaged)".to_string(), expr());
    map.insert("Karil's crossbow (Undamaged)".to_string(), expr());
    map.insert("Karil's leatherskirt (Undamaged)".to_string(), expr());
    map.insert("Karil's leathertop (Undamaged)".to_string(), expr());

    map.insert("Torag's hammers (Undamaged)".to_string(), expr());
    map.insert("Torag's helm (Undamaged)".to_string(), expr());
    map.insert("Torag's platebody (Undamaged)".to_string(), expr());
    map.insert("Torag's platelegs (Undamaged)".to_string(), expr());

    map.insert("Verac's brassard (Undamaged)".to_string(), expr());
    map.insert("Verac's flail (Undamaged)".to_string(), expr());
    map.insert("Verac's helm (Undamaged)".to_string(), expr());
    map.insert("Verac's plateskirt (Undamaged)".to_string(), expr());
}

fn add_guardians(map: &mut ExprMap) {
    map.insert("Granite maul (Normal)".to_string(), expr());
    map.insert("Granite gloves".to_string(), expr());
    map.insert("Granite ring".to_string(), expr());
    map.insert("Granite hammer".to_string(), expr());

    let mut expr = BoolExpr::builder();
    expr.var(Region::Morytania);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Guardian boots".to_string(), expr);
}

fn add_tob(map: &mut ExprMap) {
    map.insert("Ghrazi rapier".to_string(), expr());
    map.insert("Sanguinesti staff (Charged)".to_string(), expr());
    map.insert("Scythe of vitur (Charged)".to_string(), expr());

    map.insert("Justiciar faceguard".to_string(), expr());
    map.insert("Justiciar chestguard".to_string(), expr());
    map.insert("Justiciar legguards".to_string(), expr());

    let mut expr = BoolExpr::builder();
    expr.var(Region::Morytania);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Avernic defender (Normal)".to_string(), expr);
}

fn add_nightmare(map: &mut ExprMap) {
    map.insert("Nightmare staff".to_string(), expr());

    map.insert("Inquisitor's great helm".to_string(), expr());
    map.insert("Inquisitor's hauberk".to_string(), expr());
    map.insert("Inquisitor's plateskirt".to_string(), expr());
    map.insert("Inquisitor's mace".to_string(), expr());
}

fn add_other(map: &mut ExprMap) {
    map.insert("Black mask (Uncharged)".to_string(), expr());
    map.insert("Slayer helmet".to_string(), expr());
    map.insert("Slayer helmet (i)".to_string(), expr());
    map.insert("Amulet of blood fury".to_string(), expr());
}
