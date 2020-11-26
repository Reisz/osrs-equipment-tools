//! Filters for unnecessary items.

pub mod barrows;
pub mod charges;
pub mod cosmetic;
pub mod dmm;
pub mod fire_arrow;
pub mod poison;
pub mod suffixes;

use std::{collections::HashSet, sync::Mutex};

use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

lazy_static! {
    /// Set of item names to be removed.
    static ref NAME_SET: Mutex<HashSet<String>> = Mutex::new({
        let mut set = HashSet::new();

        barrows::add_names(&mut set);
        charges::add_names(&mut set);
        cosmetic::add_names(&mut set);
        dmm::add_names(&mut set);
        fire_arrow::add_names(&mut set);
        poison::add_names(&mut set);

        set
    });
}

/// Return true if no filter applies for the item.
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
    if NAME_SET.lock().unwrap().remove(&item.name) {
        return false;
    }

    // Remove items with filtered suffix
    if suffixes::has_filtered_suffix(&item.name) {
        return false;
    }

    true
}

/// Print leftover names in the item filter (probably typos).
pub fn check() {
    for name in NAME_SET.lock().unwrap().iter() {
        println!("Missed name filter: {}", name);
    }
}
