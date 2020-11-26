//! Filter out damaged variants of barrows pieces.

use std::collections::HashSet;

/// Filter out the 5 damage variants for each piece of [Barrows equipment].
///
///[Barrows equipment]: https://oldschool.runescape.wiki/w/Barrows_equipment
pub fn add_barrows(list: &mut HashSet<String>) {
    add_variants("Ahrim's hood", list);
    add_variants("Ahrim's robeskirt", list);
    add_variants("Ahrim's robetop", list);
    add_variants("Ahrim's staff", list);

    add_variants("Dharok's greataxe", list);
    add_variants("Dharok's helm", list);
    add_variants("Dharok's platebody", list);
    add_variants("Dharok's platelegs", list);

    add_variants("Guthan's chainskirt", list);
    add_variants("Guthan's helm", list);
    add_variants("Guthan's platebody", list);
    add_variants("Guthan's warspear", list);

    add_variants("Karil's coif", list);
    add_variants("Karil's crossbow", list);
    add_variants("Karil's leatherskirt", list);
    add_variants("Karil's leathertop", list);

    add_variants("Torag's hammers", list);
    add_variants("Torag's helm", list);
    add_variants("Torag's platebody", list);
    add_variants("Torag's platelegs", list);

    add_variants("Verac's brassard", list);
    add_variants("Verac's flail", list);
    add_variants("Verac's helm", list);
    add_variants("Verac's plateskirt", list);
}

pub fn add_variants(name: &str, list: &mut HashSet<String>) {
    list.insert(format!("{} 100", name));
    list.insert(format!("{} 75", name));
    list.insert(format!("{} 50", name));
    list.insert(format!("{} 25", name));
    list.insert(format!("{} 0", name));
}
