//! Aggregate a set of items with the same stats into a single stand-in item.

use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

pub mod broodoo;
pub mod capes;
pub mod gods;
pub mod hunter_gear;
pub mod other_gods;
pub mod skill_capes;

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    broodoo::add_filter_names(set);
    gods::add_filter_names(set);
    hunter_gear::add_filter_names(set);
    other_gods::add_filter_names(set);
    skill_capes::add_filter_names(set);
    capes::add_filter_names(set);
}

/// Performs aggregation.
pub trait Aggregator {
    /// Aggregate one item.
    fn aggregate(&self, item: &mut ItemProperties);
}
type AggregationMap = HashMap<String, Box<dyn Aggregator + Send>>;

lazy_static! {
    /// Set of wiki names of items to be aggregated.
    static ref WIKI_NAMES: Mutex<AggregationMap> = Mutex::new({
        let mut map = HashMap::new();

        broodoo::add_aggregators(&mut map);
        gods::add_aggregators(&mut map);
        hunter_gear::add_aggregators(&mut map);
        other_gods::add_aggregators(&mut map);
        skill_capes::add_aggregators(&mut map);
        capes::add_aggregators(&mut map);

        map
    });
}
/// Process the items which are kept.
pub fn apply_aggregation(item: &mut ItemProperties) {
    if let Some(agg) = WIKI_NAMES
        .lock()
        .unwrap()
        .remove(item.wiki_name.as_ref().unwrap())
    {
        agg.aggregate(item);
    }
}

/// Print leftover names in the item list (probably typos).
pub fn check() {
    for (name, _) in WIKI_NAMES.lock().unwrap().iter() {
        println!("Error: Missed aggregation: {}", name);
    }
}
