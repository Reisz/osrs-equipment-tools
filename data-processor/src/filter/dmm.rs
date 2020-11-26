//! Filter out DMM-only items.

use std::collections::HashSet;

/// Removes the following DMM-only item sets:
/// - [Deadman armour](https://oldschool.runescape.wiki/w/Deadman_armour)
/// - [Ancient Warriors' equipment](https://oldschool.runescape.wiki/w/Ancient_Warriors%27_equipment)
/// - [Deadman starter pack](https://oldschool.runescape.wiki/w/Deadman_starter_pack)
pub fn add_names(set: &mut HashSet<String>) {
    set.insert("Deadman's chest".to_string());
    set.insert("Deadman's legs".to_string());
    set.insert("Deadman's cape".to_string());

    set.insert("Statius's full helm".to_string());
    set.insert("Statius's platebody".to_string());
    set.insert("Statius's platelegs".to_string());
    set.insert("Statius's warhammer".to_string());

    set.insert("Vesta's chainbody".to_string());
    set.insert("Vesta's plateskirt".to_string());
    set.insert("Vesta's longsword".to_string());
    set.insert("Vesta's spear".to_string());

    set.insert("Zuriel's hood".to_string());
    set.insert("Zuriel's robe bottom".to_string());
    set.insert("Zuriel's robe top".to_string());
    set.insert("Zuriel's staff".to_string());

    set.insert("Morrigan's coif".to_string());
    set.insert("Morrigan's leather body".to_string());
    set.insert("Morrigan's leather chaps".to_string());
    set.insert("Morrigan's javelin".to_string());
    set.insert("Morrigan's throwing axe".to_string());

    set.insert("Starter sword".to_string());
    set.insert("Starter bow".to_string());
    set.insert("Starter staff".to_string());
}
