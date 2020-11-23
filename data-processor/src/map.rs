//! Adjust item data and apply projection.

use data::Item;

use crate::osrsbox::ItemProperties;

/// Apply all transformation methods.
pub fn map(item: ItemProperties) -> Result<Item, String> {
    item.project()
}
