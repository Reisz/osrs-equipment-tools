//! Filters for unnecessary items.

pub mod barrows;
pub mod cosmetic;
pub mod dmm;
pub mod fire_arrow;
pub mod suffixes;

use std::{collections::HashSet, sync::Mutex};

use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

fn create_filter_list() -> HashSet<String> {
    let mut list = HashSet::new();

    barrows::add_barrows(&mut list);
    cosmetic::add_cosmetics(&mut list);
    dmm::add_dmm(&mut list);
    fire_arrow::add_arrows(&mut list);

    list
}

lazy_static! {
    static ref FILTER_LIST: Mutex<HashSet<String>> = Mutex::new(create_filter_list());
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
    if FILTER_LIST.lock().unwrap().remove(&item.name) {
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
    for name in FILTER_LIST.lock().unwrap().iter() {
        println!("Missed filter: {}", name);
    }
}
