//! Filtering for [poisoned variants].
//!
//! [poisoned variants]: https://oldschool.runescape.wiki/w/Poison#Weapon_poisoning_(members_only)

use std::collections::HashSet;

const METALS: &[&str] = &[
    "Bronze", "Iron", "Steel", "Black", "Mithril", "Adamant", "Rune", "Dragon",
];
const COMMON_METALS: &[&str] = &[
    "Bronze", "Iron", "Steel", "Mithril", "Adamant", "Rune", "Dragon",
];
const BOLT_METALS: &[&str] = &[
    "Bronze", "Blurite", "Iron", "Steel", "Mithril", "Adamant", "Runite", "Dragon",
];

/// Keeps `(p++)` variants for melee weapons and unpoisoned variants for ammunition.
pub fn add_poisoned(list: &mut HashSet<String>) {
    metals("spear", METALS, spear, list);

    metals("hasta", COMMON_METALS, spear, list);

    metals("dagger", METALS, weapon, list);
    weapon("White dagger".to_string(), list);
    weapon_space("Bone dagger".to_string(), list);
    weapon("Keris".to_string(), list);
    weapon_space("Abyssal dagger".to_string(), list);

    metals("arrow", COMMON_METALS, ammo, list);
    ammo("Amethyst arrow".to_string(), list);

    metals("bolts ", BOLT_METALS, ammo, list);

    metals("dart", METALS, ammo, list);

    metals("knife", METALS, ammo, list);

    metals("javelin", COMMON_METALS, ammo, list);
    ammo("Amethyst javelin".to_string(), list);
}

fn weapon_space(s: String, list: &mut HashSet<String>) {
    list.insert(format!("{} (p)", &s));
    list.insert(format!("{} (p+)", &s));
    list.insert(s);
}

fn weapon(s: String, list: &mut HashSet<String>) {
    list.insert(format!("{}(p)", &s));
    list.insert(format!("{}(p+)", &s));
    list.insert(s);
}

fn spear(s: String, list: &mut HashSet<String>) {
    list.insert(format!("{}(p)", &s));
    list.insert(format!("{}(p+)", &s));
    list.insert(format!("{}(kp)", &s));
    list.insert(s);
}

fn ammo(s: String, list: &mut HashSet<String>) {
    list.insert(format!("{}(p)", &s));
    list.insert(format!("{}(p+)", &s));
    list.insert(format!("{}(p++)", &s));
}

fn metals<F: Fn(String, &mut HashSet<String>)>(
    name: &str,
    metals: &[&str],
    f: F,
    list: &mut HashSet<String>,
) {
    for metal in metals {
        f(format!("{} {}", metal, name), list);
    }
}
