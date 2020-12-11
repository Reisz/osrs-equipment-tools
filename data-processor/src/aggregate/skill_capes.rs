//! Consolidate [Skill capes](https://oldschool.runescape.wiki/w/Cape_of_Accomplishment)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const SKILLS: &[&str] = &[
    "Agility",
    "Attack",
    "Construct.",
    "Cooking",
    "Crafting",
    "Defence",
    "Farming",
    "Firemaking",
    "Fishing",
    "Fletching",
    "Herblore",
    "Hitpoints",
    "Hunter",
    "Magic",
    "Mining",
    "Prayer",
    "Ranging",
    "Runecraft",
    "Slayer",
    "Smithing",
    "Thieving",
    "Woodcutting",
    "Quest point",
    "Achievement diary",
    "Music",
];

const MAX_CAPES: &[(&str, &str)] = &[
    ("Accumulator", ""),
    ("Ardougne", ""),
    ("Assembler", " (Normal)"),
    ("Infernal", " (Normal)"),
    ("Fire", " (Normal)"),
    ("Mythical", ""),
    ("Imbued guthix", " (Normal)"),
    ("Imbued saradomin", " (Normal)"),
    ("Imbued zamorak", " (Normal)"),
    ("Guthix", ""),
    ("Saradomin", ""),
    ("Zamorak", ""),
];

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    set.extend(SKILLS.iter().map(|s| format!("{} cape (Untrimmed)", s)));
    set.extend(SKILLS.iter().map(|s| format!("{} cape (Trimmed)", s)));

    set.insert("Max cape".to_string());

    set.extend(
        MAX_CAPES
            .iter()
            .map(|(c, t)| format!("{} max cape{}", c, t)),
    );
}

struct Agg(&'static str);
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = format!("Cape of accomplishment{}", self.0);
        item.wiki_url =
            Some("https://oldschool.runescape.wiki/w/Cape_of_Accomplishment".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert("Strength cape (Untrimmed)".to_string(), Box::new(Agg("")));
    map.insert("Strength cape (Trimmed)".to_string(), Box::new(Agg(" (t)")));
}
