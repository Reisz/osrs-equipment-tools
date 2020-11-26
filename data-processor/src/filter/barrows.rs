//! Filter out damaged variants of barrows pieces.

use std::collections::HashSet;

/// Filter out the 5 damage variants for each piece of [Barrows equipment].
///
///[Barrows equipment]: https://oldschool.runescape.wiki/w/Barrows_equipment
pub fn add_names(set: &mut HashSet<String>) {
    add_variants("Ahrim's hood", set);
    add_variants("Ahrim's robeskirt", set);
    add_variants("Ahrim's robetop", set);
    add_variants("Ahrim's staff", set);

    add_variants("Dharok's greataxe", set);
    add_variants("Dharok's helm", set);
    add_variants("Dharok's platebody", set);
    add_variants("Dharok's platelegs", set);

    add_variants("Guthan's chainskirt", set);
    add_variants("Guthan's helm", set);
    add_variants("Guthan's platebody", set);
    add_variants("Guthan's warspear", set);

    add_variants("Karil's coif", set);
    add_variants("Karil's crossbow", set);
    add_variants("Karil's leatherskirt", set);
    add_variants("Karil's leathertop", set);

    add_variants("Torag's hammers", set);
    add_variants("Torag's helm", set);
    add_variants("Torag's platebody", set);
    add_variants("Torag's platelegs", set);

    add_variants("Verac's brassard", set);
    add_variants("Verac's flail", set);
    add_variants("Verac's helm", set);
    add_variants("Verac's plateskirt", set);
}

/// Variants `100`, `75`, `50`, `25` (`0` not equippable).
fn add_variants(name: &str, set: &mut HashSet<String>) {
    set.insert(format!("{} 100", name));
    set.insert(format!("{} 75", name));
    set.insert(format!("{} 50", name));
    set.insert(format!("{} 25", name));
}
