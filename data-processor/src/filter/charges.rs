//! Filter out items wich change based on [degradation](https://oldschool.runescape.wiki/w/Degradation)
//! or [numerical suffix](https://oldschool.runescape.wiki/w/(numerical)).

use std::collections::HashSet;

/// Removes variants with low charges of:
/// - [Dragonstone jewellery](https://oldschool.runescape.wiki/w/Dragonstone#Magic)
/// - [Pharao's sceptre](https://oldschool.runescape.wiki/w/Pharaoh's_sceptre)
/// - [Tridents & Revenant Weapons](https://oldschool.runescape.wiki/w/(uncharged))
/// - [Rod of ivandis](https://oldschool.runescape.wiki/w/Rod_of_ivandis#(10))
/// - [Void seal](https://oldschool.runescape.wiki/w/Void_seal#(8))
///
/// Removes charged variants of:
/// - [Broodoo shield](https://oldschool.runescape.wiki/w/Broodoo_shield)
/// - [Black mask](https://oldschool.runescape.wiki/w/Black_mask)
pub fn add_items(set: &mut HashSet<String>) {
    set.insert("Combat bracelet".to_string());
    add_all((1..6).map(|i| format!("Combat bracelet({})", i)), set);

    set.insert("Amulet of glory".to_string());
    add_all((1..6).map(|i| format!("Amulet of glory({})", i)), set);

    set.insert("Pharaoh's sceptre".to_string());
    add_all((1..8).map(|i| format!("Pharaoh's sceptre ({})", i)), set);

    set.insert("Uncharged trident".to_string());
    set.insert("Uncharged trident (e)".to_string());
    set.insert("Uncharged toxic trident".to_string());
    set.insert("Uncharged toxic trident (e)".to_string());

    set.insert("Craw's bow (u)".to_string());
    set.insert("Thammaron's sceptre (u)".to_string());
    set.insert("Viggora's chainmace (u)".to_string());

    add_all((1..10).map(|i| format!("Rod of ivandis ({})", i)), set);

    add_all((1..8).map(|i| format!("Void seal({})", i)), set);

    add_all((1..=10).map(|i| format!("Broodoo shield ({})", i)), set);
    add_all((1..=10).map(|i| format!("Black mask ({})", i)), set);
    add_all((1..=10).map(|i| format!("Black mask ({}) (i)", i)), set);
}

fn add_all(strings: impl Iterator<Item = String>, set: &mut HashSet<String>) {
    for s in strings {
        set.insert(s);
    }
}
