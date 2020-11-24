//! Adjust item data and apply projection.

use data::Item;
use trailblazer::data::get_expr;

use crate::osrsbox::ItemProperties;

/// Apply all transformation methods.
pub fn map(item: ItemProperties) -> Result<Item, String> {
    item.project().map(|mut i| {
        i.trailblazer = get_expr(&i.name);
        i
    })
}
