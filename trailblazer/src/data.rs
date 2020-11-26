//! Trailbalzer region data based on item names.

pub mod asgarnia;
pub mod fremennik;
pub mod morytania;
pub mod rfd;

use std::collections::HashMap;

use crate::{bool_expr::BoolExpr, vars::Region};

/// Maps item names to Trailblazer region expressions.
pub type ExprMap = HashMap<String, BoolExpr<Region>>;

/// Get a hash map conaining the region expression by item name.
pub fn create_map() -> ExprMap {
    let mut map = HashMap::new();

    rfd::add_items(&mut map);
    asgarnia::add_items(&mut map);
    fremennik::add_items(&mut map);
    morytania::add_items(&mut map);

    map
}
