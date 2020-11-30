//! Items with drop sources in multiple regions.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn and(r1: Region, r2: Region) -> BoolExpr::Region {
    let mut builder = BoolExprBuilder::new();
    builder.var(r1);
    builder.var(r2);
    builder.and();
    builder.finalize().unwrap()
}

/// Add requirements for the following items:
/// - [Dragon chainbody](https://oldschool.runescape.wiki/w/Dragon_chainbody)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Dragon chainbody".to_string(), and(Region::Desert, Region::Kandarin));
}
