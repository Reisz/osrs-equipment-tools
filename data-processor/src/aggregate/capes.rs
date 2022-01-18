//! Consolidate all capes with equivalent stats to [Black cape](https://oldschool.runescape.wiki/w/Black_cape)

use std::{collections::HashSet, hash::BuildHasher};

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const COLORS: &[&str] = &["Blue", "Green", "Orange", "Pink", "Purple", "Red", "Yellow"];
const FREMMY_COLORS: &[&str] = &[
    "cyan", "brown", "blue", "green", "red", "grey", "yellow", "teal", "purple", "pink", "black",
];

/// Add wiki names of items to filter.
pub fn add_filter_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
    for i in 1..=50 {
        set.insert(format!("Team-{} cape", i));
    }

    for name in &["i", "x", "zero"] {
        set.insert(format!("Team cape {}", name));
    }

    set.insert("Castlewars cloak (Saradomin)".to_string());
    set.insert("Castlewars cloak (Zamorak)".to_string());

    set.extend(
        FREMMY_COLORS
            .iter()
            .map(|c| format!("Fremennik {} cloak", c)),
    );
    set.extend(COLORS.iter().map(|c| format!("{} cape", c)));

    set.insert("Cabbage cape".to_string());
    set.insert("Ham cloak".to_string());
    set.insert("Fish sack".to_string());
    set.insert("Lunar cape".to_string());

    set.insert("Spotted cape".to_string());
    set.insert("Spottier cape".to_string());
}

struct Agg;
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = "Cape".to_string();
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Cape".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert("Black cape".to_string(), Box::new(Agg));
}
