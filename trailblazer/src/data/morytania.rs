//! Items mainly locked by Morytania.

use super::ExprMap;
use crate::{bool_expr::BoolExpr, vars::Region};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Morytania)
}

/// Add requirements for the following items:
/// - [Barrows equipment](https://oldschool.runescape.wiki/w/Barrows_equipment)
pub fn add_morytania(map: &mut ExprMap) {
    add_barrows(map);
}
/// Add Morytania requirement to the barrows pieces.
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
