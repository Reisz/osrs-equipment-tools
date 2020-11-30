//! Items with drop sources in multiple regions.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn or(r1: Region, r2: Region) -> BoolExpr<Region> {
    let mut builder = BoolExprBuilder::new();
    builder.var(r1);
    builder.var(r2);
    builder.or();
    builder.finalize().unwrap()
}

/// Add requirements for the following items:
/// - [Dragon chainbody](https://oldschool.runescape.wiki/w/Dragon_chainbody)
/// - [Dragon 2h](https://oldschool.runescape.wiki/w/Dragon_2h_sword)
pub fn add_items(map: &mut ExprMap) {
    map.insert(
        "Dragon chainbody".to_string(),
        or(Region::Desert, Region::Kandarin),
    );
    map.insert(
        "Dragon 2h sword".to_string(),
        or(Region::Wilderness, Region::Desert),
    );
}
