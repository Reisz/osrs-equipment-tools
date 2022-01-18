//! Filters for unnecessary items.

pub mod barrows;
pub mod charges;
pub mod cosmetic;
pub mod dmm;
pub mod fire_arrow;
pub mod heraldic;
pub mod misc;
pub mod poison;
pub mod suffixes;

use std::{collections::HashSet, sync::Mutex};

use lazy_static::lazy_static;

use crate::aggregate;
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
        misc::add_names(&mut set);
        poison::add_names(&mut set);

        set
    });

    /// Set of wiki item names to be removed.
    static ref WIKI_NAME_SET: Mutex<HashSet<String>> = Mutex::new({
        let mut set = HashSet::new();

        aggregate::add_filter_names(&mut set);

        charges::add_wiki_names(&mut set);
        cosmetic::add_wiki_names(&mut set);
        heraldic::add_wiki_names(&mut set);

        set
    });
}

/// Return true if no filter applies for the item.
///
/// # Panics
///
/// When unable to acquire the global locks.
#[must_use]
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

    // Remove items on name set
    if NAME_SET.lock().unwrap().remove(&item.name) {
        return false;
    }

    // Remove items on wiki name set
    if let Some(wiki_name) = item.wiki_name.as_ref() {
        if WIKI_NAME_SET.lock().unwrap().remove(wiki_name) {
            return false;
        }

        if suffixes::has_filtered_wiki_suffix(wiki_name) {
            return false;
        }
    }

    // Remove items with filtered suffix
    if suffixes::has_filtered_suffix(&item.name) {
        return false;
    }

    true
}

/// Print leftover names in the item filter (probably typos).
///
/// # Panics
///
/// When unable to acquire the global locks.
pub fn check() {
    for name in NAME_SET.lock().unwrap().iter() {
        println!("Error: Missed name filter: {}", name);
    }

    for name in WIKI_NAME_SET.lock().unwrap().iter() {
        println!("Error: Missed wiki name filter: {}", name);
    }
}
