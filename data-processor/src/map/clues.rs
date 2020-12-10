//! Adds clue tier data to items.

use std::{collections::HashMap, sync::Mutex};

use data::{Clue, Item};
use lazy_static::lazy_static;

lazy_static! {
    /// Set of 3rd age item names.
    static ref ITEM_NAMES: Mutex<HashMap<String, Clue>> = Mutex::new({
        let set = HashMap::new();

        set
    });
}

/// Applies `item.clue = <level>` for clue uniques.
pub fn apply_value(item: &mut Item) {
    item.clue = ITEM_NAMES.lock().unwrap().remove(&item.name);
}

/// Print leftover names in the item list (probably typos).
pub fn check() {
    for (name, _) in ITEM_NAMES.lock().unwrap().iter() {
        println!("Missed clue data: {}", name);
    }
}
