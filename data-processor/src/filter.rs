//! Filters for unnecessary items.

use crate::osrsbox::ItemProperties;

/// Return true if no filter applies.
pub fn keep(item: &ItemProperties) -> bool {
    !item.duplicate && item.equipment.is_some()
}
