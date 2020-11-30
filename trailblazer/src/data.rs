//! Trailbalzer region data based on item names.

pub mod asgarnia;
pub mod desert;
pub mod fremennik;
pub mod kandarin;
pub mod misc;
pub mod morytania;
pub mod rfd;
pub mod tirannwn;
pub mod wilderness;

use std::collections::HashMap;

use crate::{bool_expr::BoolExpr, vars::Region};

/// Maps item names to Trailblazer region expressions.
pub type ExprMap = HashMap<String, BoolExpr<Region>>;

/// Get a hash map conaining the region expression by item name.
pub fn create_map() -> ExprMap {
    let mut map = HashMap::new();

    rfd::add_items(&mut map);
    asgarnia::add_items(&mut map);
    desert::add_items(&mut map);
    fremennik::add_items(&mut map);
    kandarin::add_items(&mut map);
    misc::add_items(&mut map);
    morytania::add_items(&mut map);
    tirannwn::add_items(&mut map);
    wilderness::add_items(&mut map);

    map
}
