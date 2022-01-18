//! Filtering for [poisoned variants].
//!
//! [poisoned variants]: https://oldschool.runescape.wiki/w/Poison#Weapon_poisoning_(members_only)

use std::{collections::HashSet, hash::BuildHasher};

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
pub fn add_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
    metals("spear", METALS, spear, set);

    metals("hasta", COMMON_METALS, spear, set);

    metals("dagger", METALS, weapon, set);
    weapon("White dagger", set);
    weapon_space("Bone dagger", set);
    weapon("Keris", set);
    weapon_space("Abyssal dagger", set);

    metals("arrow", COMMON_METALS, ammo, set);
    ammo("Amethyst arrow", set);

    metals("bolts ", BOLT_METALS, ammo, set);

    metals("dart", METALS, ammo, set);

    metals("knife", METALS, ammo, set);

    metals("javelin", COMMON_METALS, ammo, set);
    ammo("Amethyst javelin", set);
}

/// Weapons with space before parentheses.
fn weapon_space<S: BuildHasher>(s: &str, set: &mut HashSet<String, S>) {
    set.insert(format!("{} (p)", &s));
    set.insert(format!("{} (p+)", &s));
    set.insert(s.to_owned());
}

/// Weapons without space before parentheses.
fn weapon<S: BuildHasher>(s: &str, set: &mut HashSet<String, S>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(s.to_owned());
}

/// Spears and hastas (includes `(kp)`).
fn spear<S: BuildHasher>(s: &str, set: &mut HashSet<String, S>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(format!("{}(kp)", &s));
    set.insert(s.to_owned());
}

/// All ammunition.
fn ammo<S: BuildHasher>(s: &str, set: &mut HashSet<String, S>) {
    set.insert(format!("{}(p)", &s));
    set.insert(format!("{}(p+)", &s));
    set.insert(format!("{}(p++)", &s));
}

/// Apply item names to a set of metal names, then apply poison level.
fn metals<S: BuildHasher, F: Fn(&str, &mut HashSet<String, S>)>(
    name: &str,
    metals: &[&str],
    f: F,
    set: &mut HashSet<String, S>,
) {
    for metal in metals {
        f(&format!("{} {}", metal, name), set);
    }
}
