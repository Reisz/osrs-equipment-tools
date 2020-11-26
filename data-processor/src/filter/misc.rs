//! Filtered items which do not fit into anther category.

use std::collections::HashSet;

/// Adds the following items:
/// - [Ice arrows](https://oldschool.runescape.wiki/w/Ice_arrows)
/// - [Lunar staff stages](https://oldschool.runescape.wiki/w/Lunar_staff_-_pt1)
pub fn add_names(set: &mut HashSet<String>) {
    set.insert("Ice arrows".to_string());
    set.insert("Lunar staff - pt1".to_string());
    set.insert("Lunar staff - pt2".to_string());
    set.insert("Lunar staff - pt3".to_string());
}
