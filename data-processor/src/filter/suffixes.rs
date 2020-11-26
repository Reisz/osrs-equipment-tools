//! Filtering based on suffix.

use std::collections::HashSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref SUFFIXES: HashSet<String> = {
        let mut set = HashSet::new();

        set.insert("(l)".to_string());

        set.insert("(basic)".to_string());
        set.insert("(attuned)".to_string());
        set.insert("(perfected)".to_string());

        set.insert("(t)".to_string());
        set.insert("(g)".to_string());
        set.insert("(or)".to_string());
        set.insert("(h1)".to_string());
        set.insert("(h2)".to_string());
        set.insert("(h3)".to_string());
        set.insert("(h4)".to_string());
        set.insert("(h5)".to_string());

        set.insert("(dark)".to_string());
        set.insert("(light)".to_string());
        set.insert("(dusk)".to_string());

        set.insert("(uncharged)".to_string());
        set.insert("(empty)".to_string());
        set.insert("(inactive)".to_string());
        set.insert("(full)".to_string());

        set.insert("(nz)".to_string());

        set
    };
}

/// Checks the [suffix] (after the last space) of the name. The following will be removed:
/// - `(l)`: [Trouver parchment lock](https://oldschool.runescape.wiki/w/Trouver_parchment)
/// - `(basic)`, `(attuned)`, `(perfected)`: [The Gauntlet](https://oldschool.runescape.wiki/w/The_Gauntlet)
/// - `(t)`, `(g)`, `(or)`, `(h*)`: [Ornamental](https://oldschool.runescape.wiki/w/Ornamental_armour)
/// - `(dark)`, `(light)`, `(dusk)`: [Mystic variants](https://oldschool.runescape.wiki/w/Mystic_robes)
/// - `(uncharged)`, `(empty)`, `(inactive)`, `(full)`: Charge-based variants
/// - `(nz)`: [Nightmare Zone](https://oldschool.runescape.wiki/w/(nz))
///
/// [suffix]: https://oldschool.runescape.wiki/w/Suffixes
pub fn has_filtered_suffix(name: &str) -> bool {
    if let Some(idx) = name.rfind(' ') {
        SUFFIXES.contains(&name[(idx + 1)..])
    } else {
        false
    }
}
