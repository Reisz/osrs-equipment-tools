//! Filtered items which do not fit into anther category.

use std::collections::HashSet;

/// Adds the following items:
/// - [Ice arrows](https://oldschool.runescape.wiki/w/Ice_arrows)
/// - [Lunar staff stages](https://oldschool.runescape.wiki/w/Lunar_staff_-_pt1)
/// - [Plateskirts](https://oldschool.runescape.wiki/w/Plateskirt)
/// - Ironman armours
///     - [Ironman armour](https://oldschool.runescape.wiki/w/Ironman_armour)
///     - [Hardcore ironman armour](https://oldschool.runescape.wiki/w/Hardcore_ironman_armour)
///     - [Ultimate ironman armour](https://oldschool.runescape.wiki/w/Ultimate_ironman_armour)
/// - [New Crystal Equipment](https://oldschool.runescape.wiki/w/Crystal_bow_(i))
pub fn add_names(set: &mut HashSet<String>) {
    set.insert("Ice arrows".to_string());

    set.insert("Lunar staff - pt1".to_string());
    set.insert("Lunar staff - pt2".to_string());
    set.insert("Lunar staff - pt3".to_string());

    set.insert("New crystal bow (i)".to_string());
    set.insert("New crystal shield (i)".to_string());
    set.insert("New crystal halberd full (i)".to_string());

    add_plateskirts(set);
    add_ironman(set);
}

fn add_plateskirts(set: &mut HashSet<String>) {
    set.insert("Bronze plateskirt".to_string());
    set.insert("Iron plateskirt".to_string());
    set.insert("Steel plateskirt".to_string());
    set.insert("Black plateskirt".to_string());
    set.insert("White plateskirt".to_string());
    set.insert("Mithril plateskirt".to_string());
    set.insert("Adamant plateskirt".to_string());
    set.insert("Rune plateskirt".to_string());
    set.insert("Dragon plateskirt".to_string());

    set.insert("Saradomin plateskirt".to_string());
    set.insert("Guthix plateskirt".to_string());
    set.insert("Zamorak plateskirt".to_string());
    set.insert("Armadyl plateskirt".to_string());
    set.insert("Bandos plateskirt".to_string());
    set.insert("Ancient plateskirt".to_string());

    set.insert("Proselyte tasset".to_string());
}

fn add_ironman(set: &mut HashSet<String>) {
    set.insert("Ironman helm".to_string());
    set.insert("Ironman platebody".to_string());
    set.insert("Ironman platelegs".to_string());

    set.insert("Hardcore ironman helm".to_string());
    set.insert("Hardcore ironman platebody".to_string());
    set.insert("Hardcore ironman platelegs".to_string());

    set.insert("Ultimate ironman helm".to_string());
    set.insert("Ultimate ironman platebody".to_string());
    set.insert("Ultimate ironman platelegs".to_string());
}
