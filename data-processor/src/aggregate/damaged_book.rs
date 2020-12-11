//! Consolidate [Damaged books](https://oldschool.runescape.wiki/w/Damaged_book)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    set.insert("Damaged book (Ancient)".to_string());
    set.insert("Damaged book (Armadyl)".to_string());
    set.insert("Damaged book (Bandos)".to_string());
    set.insert("Damaged book (Guthix)".to_string());
    set.insert("Damaged book (Zamorak)".to_string());
}

struct Agg;
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.wiki_url = Some("https://oldschool.runescape.wiki/w/Damaged_book".to_string());
    }
}

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert("Damaged book (Saradomin)".to_string(), Box::new(Agg));
}
