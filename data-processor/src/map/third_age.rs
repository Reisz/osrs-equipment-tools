//! Mark [3rd age equipment](https://oldschool.runescape.wiki/w/3rd_age_equipment) as such.
//!
//! Allows filtering them out later.

use std::{collections::HashSet, sync::Mutex};

use data::Item;
use lazy_static::lazy_static;

lazy_static! {
    /// Set of 3rd age item names.
    static ref ITEM_NAMES: Mutex<HashSet<String>> = Mutex::new({
        let mut set = HashSet::new();

        set.insert("3rd age full helmet".to_string());
        set.insert("3rd age kiteshield".to_string());
        set.insert("3rd age platebody".to_string());
        set.insert("3rd age plateskirt".to_string());
        set.insert("3rd age platelegs".to_string());
        set.insert("3rd age longsword".to_string());

        set.insert("3rd age mage hat".to_string());
        set.insert("3rd age robe".to_string());
        set.insert("3rd age robe top".to_string());
        set.insert("3rd age amulet".to_string());
        set.insert("3rd age wand".to_string());

        set.insert("3rd age range coif".to_string());
        set.insert("3rd age range legs".to_string());
        set.insert("3rd age range top".to_string());
        set.insert("3rd age vambraces".to_string());
        set.insert("3rd age bow".to_string());

        set.insert("3rd age druidic cloak".to_string());
        set.insert("3rd age druidic robe bottoms".to_string());
        set.insert("3rd age druidic robe top".to_string());
        set.insert("3rd age druidic staff".to_string());

        set.insert("3rd age axe".to_string());
        set.insert("3rd age cloak".to_string());
        set.insert("3rd age pickaxe".to_string());

        set
    });
}

/// Applies `item.third_age = true` for 3rd age items.
pub fn apply_value(item: &mut Item) {
    item.third_age = ITEM_NAMES.lock().unwrap().remove(&item.name);
}

/// Print leftover names in the item list (probably typos).
pub fn check() {
    for name in ITEM_NAMES.lock().unwrap().iter() {
        println!("Error: Missed 3rd age: {}", name);
    }
}
