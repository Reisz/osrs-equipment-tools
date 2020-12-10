//! Aggregate a set of items with the same stats into a single stand-in item.

use std::collections::HashSet;

use crate::osrsbox::ItemProperties;

pub mod team_capes;

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    team_capes::add_filter_names(set);
}

/// Process the items which are kept.
pub fn apply_aggregation(item: &mut ItemProperties) {
    match item.wiki_name.as_ref().unwrap().as_str() {
        team_capes::AGGREGATE_NAME => team_capes::apply_aggregation(item),
        _ => {}
    }
}
