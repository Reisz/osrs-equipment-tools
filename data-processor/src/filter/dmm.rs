//! Filter out DMM-only items.

use std::collections::HashSet;

/// Removes the following DMM-only item sets
/// - [Deadman armour](https://oldschool.runescape.wiki/w/Deadman_armour)
/// - [Ancient Warriors' equipment](https://oldschool.runescape.wiki/w/Ancient_Warriors%27_equipment)
pub fn add_dmm(list: &mut HashSet<String>) {
    list.insert("Deaman's chest".to_string());
    list.insert("Deaman's legs".to_string());
    list.insert("Deaman's cape".to_string());

    list.insert("Statius's full helm".to_string());
    list.insert("Statius's platebody".to_string());
    list.insert("Statius's platelegs".to_string());
    list.insert("Statius's warhammer".to_string());

    list.insert("Vesta's chainbody".to_string());
    list.insert("Vesta's plateskirt".to_string());
    list.insert("Vesta's longsword".to_string());
    list.insert("Vesta's spear".to_string());

    list.insert("Zuriel's hood".to_string());
    list.insert("Zuriel's robe bottom".to_string());
    list.insert("Zuriel's robe top".to_string());
    list.insert("Zuriel's staff".to_string());

    list.insert("Morrigan's coif".to_string());
    list.insert("Morrigan's leather body ".to_string());
    list.insert("Morrigan's leather chaps".to_string());
    list.insert("Morrigan's javelin".to_string());
    list.insert("Morrigan's throwing axe".to_string());
}
