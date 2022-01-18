//! Consolidate [Hunter gear](https://oldschool.runescape.wiki/w/Hunter_gear)

use std::{collections::HashSet, hash::BuildHasher};

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

/// Add wiki names of items to filter.
pub fn add_filter_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
    for area in &["Polar", "Wood", "Jungle", "Desert"] {
        set.insert(format!("{} camo top", area));
        set.insert(format!("{} camo legs", area));
    }

    for animal in &["Graahk", "Kyatt"] {
        set.insert(format!("{} top", animal));
        set.insert(format!("{} legs", animal));
    }
}

struct Agg {
    name: &'static str,
}
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = self.name.to_string();
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Hunter_gear".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert(
        "Larupia hat".to_string(),
        Box::new(Agg {
            name: "Hunter camo hat",
        }),
    );
    map.insert(
        "Larupia top".to_string(),
        Box::new(Agg {
            name: "Hunter camo top",
        }),
    );
    map.insert(
        "Larupia legs".to_string(),
        Box::new(Agg {
            name: "Hunter camo legs",
        }),
    );
}
