//! Consolidate [Broodoo shields](https://oldschool.runescape.wiki/w/Broodoo_shield)

use std::{collections::HashSet, hash::BuildHasher};

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

/// Add wiki names of items to filter.
pub fn add_filter_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
    set.insert("Broodoo shield (orange) (Uncharged)".to_string());
    set.insert("Broodoo shield (green) (Uncharged)".to_string());
}

struct Agg;
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = "Broodoo shield".to_string();
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Broodoo_shield".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert(
        "Broodoo shield (blue) (Uncharged)".to_string(),
        Box::new(Agg),
    );
}
