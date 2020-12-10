//! Consolidate [Team capes](https://oldschool.runescape.wiki/w/Team_cape)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
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

struct Agg;
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = "Team cape".to_string();
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Team_cape".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert("Team-1 cape".to_string(), Box::new(Agg));
}
