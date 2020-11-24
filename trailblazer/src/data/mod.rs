//! Trailbalzer region data based on item names.

use crate::{bool_expr::BoolExpr, vars::Region};

/// Get the restriction expression for the item with the given `name`.
pub fn get_expr(_name: &str) -> Option<BoolExpr<Region>> {
    None
}
