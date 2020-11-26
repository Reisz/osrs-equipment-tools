//! Filter out DMM-only items.

use std::collections::HashSet;

/// [Fire arrows](https://oldschool.runescape.wiki/w/Fire_arrows) and their `(lit)` variants from
/// Underground Pass.
pub fn add_arrows(list: &mut HashSet<String>) {
    add_variants("Bronze", list);
    add_variants("Iron", list);
    add_variants("Steel", list);
    add_variants("Mithril", list);
    add_variants("Adamant", list);
    add_variants("Rune", list);
    add_variants("Amethyst", list);
    add_variants("Dragon", list);
}

fn add_variants(name: &str, list: &mut HashSet<String>) {
    list.insert(format!("{} fire arrow", name));
    list.insert(format!("{} fire arrow (lit)", name));
}
