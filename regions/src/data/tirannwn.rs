//! Items mainly locked by Tirannwn.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Tirannwn)
}

fn expr_and(region: Region) -> BoolExpr<Region> {
    let mut builder = BoolExprBuilder::new();
    builder.var(Region::Tirannwn);
    builder.var(region);
    builder.and();
    builder.finalize().unwrap()
}

/// Add requirements for the following items:
/// - [Gauntlet rewards](https://oldschool.runescape.wiki/w/The_Gauntlet#Unique_Rewards_(Normal_Mode))
/// - [Zalcano drops](https://oldschool.runescape.wiki/w/Zalcano)
/// - [Zulrah drops](https://oldschool.runescape.wiki/w/Zulrah)
/// - [Dark bow](https://oldschool.runescape.wiki/w/Dark_bow)
/// - [Dragonstone gauntlets](https://oldschool.runescape.wiki/w/Dragonstone_gauntlets)
pub fn add_items(map: &mut ExprMap) {
    add_gauntlet(map);
    add_zalcano(map);
    add_zulrah(map);

    map.insert("Dark bow".to_string(), expr());
    map.insert("Dragonstone gauntlets".to_string(), expr());
}

fn add_gauntlet(map: &mut ExprMap) {
    map.insert("Crystal helm".to_string(), expr());
    map.insert("Crystal legs".to_string(), expr());
    map.insert("Crystal body".to_string(), expr());

    map.insert("Crystal bow".to_string(), expr());
    map.insert("Crystal halberd".to_string(), expr());
    map.insert("Crystal shield".to_string(), expr());

    map.insert("Blade of saeldor".to_string(), expr());
}

fn add_zalcano(map: &mut ExprMap) {
    map.insert("Crystal axe".to_string(), expr_and(Region::Fremennik));
    map.insert("Crystal harpoon".to_string(), BoolExpr::new_false());
    map.insert("Crystal pickaxe".to_string(), expr_and(Region::Wilderness));
}

fn add_zulrah(map: &mut ExprMap) {
    map.insert("Toxic blowpipe".to_string(), expr());
    map.insert(
        "Toxic staff of the dead".to_string(),
        expr_and(Region::Asgarnia),
    );
    map.insert(
        "Trident of the swamp".to_string(),
        expr_and(Region::Kandarin),
    );
    map.insert("Serpentine helm".to_string(), expr());
}
