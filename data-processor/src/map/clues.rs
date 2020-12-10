//! Adds clue tier data to items.

use std::{collections::HashMap, sync::Mutex};

use data::{Clue, Item};
use lazy_static::lazy_static;

lazy_static! {
    /// Set of 3rd age item names.
    static ref ITEM_NAMES: Mutex<HashMap<String, Clue>> = Mutex::new({
        let mut map = HashMap::new();

        // Easy
        map.insert("Vestment robe top".to_string(), Clue::Easy);
        map.insert("Vestment robe legs".to_string(), Clue::Easy);

        // Medium
        map.insert("Ranger boots".to_string(), Clue::Medium);
        map.insert("Pegasian boots".to_string(), Clue::Medium);
        map.insert("Wizard boots".to_string(), Clue::Medium);
        map.insert("Holy sandals".to_string(), Clue::Medium);
        map.insert("Devout boots".to_string(), Clue::Medium);
        map.insert("Spiked manacles".to_string(), Clue::Medium);

        map.insert("Mitre".to_string(), Clue::Medium);
        map.insert("Vestment cloak".to_string(), Clue::Medium);
        map.insert("Stole".to_string(), Clue::Medium);
        map.insert("Crozier".to_string(), Clue::Medium);

        // Hard
        map.insert("Robin hood hat".to_string(), Clue::Hard);

        // Some region combinations do not have access to mystic hat or top
        #[cfg(not(feature = "trailblazer"))]
        {
            map.insert("Enchanted hat".to_string(), Clue::Hard);
            map.insert("Enchanted top".to_string(), Clue::Hard);
        }

        map.insert("God full helm".to_string(), Clue::Hard);
        map.insert("God kiteshield".to_string(), Clue::Hard);
        map.insert("God platebody".to_string(), Clue::Hard);
        map.insert("God platelegs".to_string(), Clue::Hard);
        map.insert("God plateskirt".to_string(), Clue::Hard);

        map.insert("Blessed bracers".to_string(), Clue::Hard);
        map.insert("Blessed chaps".to_string(), Clue::Hard);
        map.insert("Blessed coif".to_string(), Clue::Hard);
        map.insert("Blessed d'hide body".to_string(), Clue::Hard);
        map.insert("Blessed d'hide boots".to_string(), Clue::Hard);
        map.insert("Blessed d'hide shield".to_string(), Clue::Hard);

        map.insert("3rd age full helmet".to_string(), Clue::Hard);
        map.insert("3rd age platebody".to_string(), Clue::Hard);
        map.insert("3rd age platelegs".to_string(), Clue::Hard);
        map.insert("3rd age plateskirt".to_string(), Clue::Hard);
        map.insert("3rd age kiteshield".to_string(), Clue::Hard);
        map.insert("3rd age range coif".to_string(), Clue::Hard);
        map.insert("3rd age range top".to_string(), Clue::Hard);
        map.insert("3rd age range legs".to_string(), Clue::Hard);
        map.insert("3rd age vambraces".to_string(), Clue::Hard);
        map.insert("3rd age mage hat".to_string(), Clue::Hard);
        map.insert("3rd age robe top".to_string(), Clue::Hard);
        map.insert("3rd age robe".to_string(), Clue::Hard);
        map.insert("3rd age amulet".to_string(), Clue::Hard);

        // Elite
        map.insert("Rangers' tunic".to_string(), Clue::Elite);
        map.insert("Ranger gloves".to_string(), Clue::Elite);
        map.insert("Holy wraps".to_string(), Clue::Elite);
        map.insert("Fremennik kilt".to_string(), Clue::Elite);

        map.insert("3rd age longsword".to_string(), Clue::Elite);
        map.insert("3rd age wand".to_string(), Clue::Elite);
        map.insert("3rd age cloak".to_string(), Clue::Elite);
        map.insert("3rd age bow".to_string(), Clue::Elite);

        // Master
        map.insert("3rd age druidic robe top".to_string(), Clue::Master);
        map.insert("3rd age druidic robe bottoms".to_string(), Clue::Master);
        map.insert("3rd age druidic cloak".to_string(), Clue::Master);
        map.insert("3rd age druidic staff".to_string(), Clue::Master);
        map.insert("3rd age pickaxe".to_string(), Clue::Master);
        map.insert("3rd age axe".to_string(), Clue::Master);

        map
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
