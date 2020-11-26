//! Filter out DMM-only items.

use std::collections::HashSet;

/// [Fire arrows](https://oldschool.runescape.wiki/w/Fire_arrows) and their `(lit)` variants from
/// Underground Pass.
pub fn add_names(set: &mut HashSet<String>) {
    add_variants("Bronze", set);
    add_variants("Iron", set);
    add_variants("Steel", set);
    add_variants("Mithril", set);
    add_variants("Adamant", set);
    add_variants("Rune", set);
    add_variants("Amethyst", set);
    add_variants("Dragon", set);
}

/// Regular and lit variants.
fn add_variants(name: &str, set: &mut HashSet<String>) {
    set.insert(format!("{} fire arrow", name));
    set.insert(format!("{} fire arrow (lit)", name));
}
