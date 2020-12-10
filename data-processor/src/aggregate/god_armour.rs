//! Consolidate [Rune god armour](https://oldschool.runescape.wiki/w/Rune_god_armour)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const FILTERED_GODS: &[&str] = &["Guthix", "Zamorak", "Armadyl", "Bandos", "Ancient"];

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    for god in FILTERED_GODS {
        set.insert(format!("{} full helm", god));
        set.insert(format!("{} kiteshield", god));
        set.insert(format!("{} platebody", god));
        set.insert(format!("{} platelegs", god));
        set.insert(format!("{} plateskirt", god));
    }
}

struct Agg {
    name: &'static str,
    wiki_url: &'static str,
}
impl Aggregator for Agg {
    fn aggregate(&self, item: &mut ItemProperties) {
        item.name = self.name.to_string();
        item.wiki_url = Some(self.wiki_url.to_string());
    }
}

const URL: &str = "https://oldschool.runescape.wiki/w/Rune_god_armour";

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert(
        "Saradomin full helm".to_string(),
        Box::new(Agg {
            name: "God full helm",
            wiki_url: URL,
        }),
    );
    map.insert(
        "Saradomin kiteshield".to_string(),
        Box::new(Agg {
            name: "God kiteshield",
            wiki_url: URL,
        }),
    );
    map.insert(
        "Saradomin platebody".to_string(),
        Box::new(Agg {
            name: "God platebody",
            wiki_url: URL,
        }),
    );
    map.insert(
        "Saradomin platelegs".to_string(),
        Box::new(Agg {
            name: "God platelegs",
            wiki_url: URL,
        }),
    );
    map.insert(
        "Saradomin plateskirt".to_string(),
        Box::new(Agg {
            name: "God plateskirt",
            wiki_url: URL,
        }),
    );
}
