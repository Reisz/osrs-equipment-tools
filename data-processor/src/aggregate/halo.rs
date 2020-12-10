//! Consolidate [Halos](https://oldschool.runescape.wiki/w/Halo)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    set.insert("Ancient halo (Normal)".to_string());
    set.insert("Armadyl halo (Normal)".to_string());
    set.insert("Bandos halo (Normal)".to_string());
    set.insert("Brassica halo (Normal)".to_string());
    set.insert("Guthix halo (Normal)".to_string());
    set.insert("Seren halo (Normal)".to_string());
    set.insert("Zamorak halo (Normal)".to_string());
}

struct Agg;
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = "Halo".to_string();
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Halo".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert("Saradomin halo (Normal)".to_string(), Box::new(Agg));
}
