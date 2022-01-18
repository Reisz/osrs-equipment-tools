//! Filter out DMM-only items.

use std::{collections::HashSet, hash::BuildHasher};

/// [Fire arrows](https://oldschool.runescape.wiki/w/Fire_arrows) and their `(lit)` variants from
/// Underground Pass.
pub fn add_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
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
fn add_variants<S: BuildHasher>(name: &str, set: &mut HashSet<String, S>) {
    set.insert(format!("{} fire arrow", name));
    set.insert(format!("{} fire arrow (lit)", name));
}
