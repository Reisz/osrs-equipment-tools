//! Filters for unnecessary items.

pub mod barrows;
pub mod dmm;

use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

fn create_filter_list() -> HashSet<String> {
    let mut list = HashSet::new();

    dmm::add_dmm(&mut list);
    barrows::add_barrows(&mut list);

    list
}

lazy_static! {
    static ref FILTER_LIST: HashSet<String> = create_filter_list();
}

/// Return true if no filter applies.
pub fn keep(item: &ItemProperties) -> bool {
    // Remove duplicates
    if item.duplicate {
        return false;
    }

    // Remove non-equipment and equipment with no positive stats
    if let Some(equipment) = item.equipment.as_ref() {
        if !equipment.has_positive() {
            return false;
        }
    } else {
        return false;
    }

    // Remove items on list
    if FILTER_LIST.contains(&item.name) {
        return false;
    }

    true
}
