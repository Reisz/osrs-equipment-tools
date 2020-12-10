//! Consolidate [Team capes](https://oldschool.runescape.wiki/w/Team_cape)

use std::collections::HashSet;

use crate::osrsbox::ItemProperties;

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    for i in 2..=50 {
        set.insert(format!("Team-{} cape", i));
    }

    for name in &["i", "x", "zero"] {
        set.insert(format!("Team cape {}", name));
    }
}

/// Name of the item to process.
pub const AGGREGATE_NAME: &str = "Team-1 cape";
const NAME: &str = "Team cape";
const WIKI_URL: &str = "https://oldschool.runescape.wiki/w/Team_cape";

/// Process the item which is kept.
pub fn apply_aggregation(item: &mut ItemProperties) {
    item.name = NAME.to_string();
    item.wiki_url = Some(WIKI_URL.to_string());
}
