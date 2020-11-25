//! Trailbalzer region data based on item names.

mod rfd;

use std::collections::HashMap;

use crate::{bool_expr::BoolExpr, vars::Region};

/// Maps item names to Trailblazer region expressions.
pub type ExprMap = HashMap<String, BoolExpr<Region>>;

/// Get a hash map conaining the region expression by item name.
pub fn create_map() -> ExprMap {
    let mut map = HashMap::new();

    rfd::add_gloves(&mut map);

    map
}
