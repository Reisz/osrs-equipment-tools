//! Items mainly locked by the desert.

use super::ExprMap;
use crate::{bool_expr::BoolExpr, vars::Region};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Desert)
}

/// Add requirements for the following items:
/// - [Pharaoh's sceptre](https://oldschool.runescape.wiki/w/Pharaoh%27s_sceptre#(8))
/// - [Keris](https://oldschool.runescape.wiki/w/Keris)
/// - [Mage training arena](https://oldschool.runescape.wiki/w/Mage_Training_Arena#Main)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Pharaoh's sceptre (8)".to_string(), expr());
    map.insert("Keris(p++)".to_string(), expr());

    add_mta(map);
}

fn add_mta(map: &mut ExprMap) {
    map.insert("Infinity gloves".to_string(), expr());
    map.insert("Infinity hat".to_string(), expr());
    map.insert("Infinity top".to_string(), expr());
    map.insert("Infinity bottoms".to_string(), expr());
    map.insert("Infinity boots".to_string(), expr());

    map.insert("Beginner wand".to_string(), expr());
    map.insert("Apprentice wand".to_string(), expr());
    map.insert("Teacher wand".to_string(), expr());
    map.insert("Master wand".to_string(), expr());

    map.insert("Mage's book".to_string(), expr());
}
