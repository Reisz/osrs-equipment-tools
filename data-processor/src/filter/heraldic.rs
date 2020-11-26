//! Filters for painted [heraldic](https://oldschool.runescape.wiki/w/Heraldic_armour) equipment.

use std::collections::HashSet;

const CRESTS: &[&str] = &[
    "Arrav",
    "Asgarnia",
    "Dorgeshuun",
    "Dragon",
    "Fairy",
    "Guthix",
    "HAM",
    "Horse",
    "Jogre",
    "Kandarin",
    "Misthalin",
    "Money",
    "Saradomin",
    "Skull",
    "Varrock",
    "Zamorak",
];

/// Adds filters for:
/// - [Heraldic helmet](https://oldschool.runescape.wiki/w/Heraldic_helmet_(Construction))
/// - [Heraldic kiteshield](https://oldschool.runescape.wiki/w/Heraldic_kiteshield_(Construction))
pub fn add_wiki_names(set: &mut HashSet<String>) {
    set.extend(
        CRESTS
            .iter()
            .map(|c| format!("Steel heraldic helm ({})", c)),
    );
    set.extend(
        CRESTS
            .iter()
            .map(|c| format!("Adamant heraldic helm ({})", c)),
    );
    set.extend(CRESTS.iter().map(|c| format!("Rune heraldic helm ({})", c)));

    set.extend(CRESTS.iter().map(|c| format!("Steel kiteshield ({})", c)));
    set.extend(CRESTS.iter().map(|c| format!("Adamant kiteshield ({})", c)));
    set.extend(CRESTS.iter().map(|c| format!("Rune kiteshield ({})", c)));
}
