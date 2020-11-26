//! Filtering for [poisoned variants].
//!
//! [poisoned variants]: https://oldschool.runescape.wiki/w/Poison#Weapon_poisoning_(members_only)

use std::collections::HashSet;

/// Smithable metals, black and dragon.
const METALS: &[&str] = &[
    "Bronze", "Iron", "Steel", "Black", "Mithril", "Adamant", "Rune", "Dragon",
];

/// Smithable metals and dragon.
const COMMON_METALS: &[&str] = &[
    "Bronze", "Iron", "Steel", "Mithril", "Adamant", "Rune", "Dragon",
];

/// Smithable metals and blurite. Rune is called runite.
const BOLT_METALS: &[&str] = &[
    "Bronze", "Blurite", "Iron", "Steel", "Mithril", "Adamant", "Runite", "Dragon",
];

/// Keeps `(p++)` variants for melee weapons and unpoisoned variants for ammunition.
pub fn add_names(set: &mut HashSet<String>) {
    metals("spear", METALS, spear, set);

    metals("hasta", COMMON_METALS, spear, set);

    metals("dagger", METALS, weapon, set);
    weapon("White dagger".to_string(), set);
    weapon_space("Bone dagger".to_string(), set);
    weapon("Keris".to_string(), set);
    weapon_space("Abyssal dagger".to_string(), set);

    metals("arrow", COMMON_METALS, ammo, set);
    ammo("Amethyst arrow".to_string(), set);

    metals("bolts ", BOLT_METALS, ammo, set);

    metals("dart", METALS, ammo, set);

    metals("knife", METALS, ammo, set);

    metals("javelin", COMMON_METALS, ammo, set);
    ammo("Amethyst javelin".to_string(), set);
}

/// Weapons with space before parentheses.
fn weapon_space(s: String, set: &mut HashSet<String>) {
    set.insert(format!("{} (p)", &s));
    set.insert(format!("{} (p+)", &s));
    set.insert(s);
}

/// Weapons without space before parentheses.
fn weapon(s: String, set: &mut HashSet<String>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(s);
}

/// Spears and hastas (includes `(kp)`).
fn spear(s: String, set: &mut HashSet<String>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(format!("{}(kp)", &s));
    set.insert(s);
}

/// All ammunition.
fn ammo(s: String, set: &mut HashSet<String>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(format!("{}(p++)", &s));
}

/// Apply item names to a set of metal names, then apply poison level.
fn metals<F: Fn(String, &mut HashSet<String>)>(
    name: &str,
    metals: &[&str],
    f: F,
    set: &mut HashSet<String>,
) {
    for metal in metals {
        f(format!("{} {}", metal, name), set);
    }
}
