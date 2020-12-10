//! Consolidate [Blessed dragonhide armour](https://oldschool.runescape.wiki/w/Blessed_dragonhide_armour)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const FILTERED_GODS: &[&str] = &["Guthix", "Zamorak", "Armadyl", "Bandos", "Ancient"];

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    for god in FILTERED_GODS {
        set.insert(format!("{} bracers", god));
        set.insert(format!("{} chaps", god));
        set.insert(format!("{} coif", god));
        set.insert(format!("{} d'hide body", god));
        set.insert(format!("{} d'hide boots", god));
        set.insert(format!("{} d'hide shield", god));
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

/// Add aggregation instructions to the map.
pub fn add_aggregators(map: &mut AggregationMap) {
    map.insert(
        "Saradomin bracers".to_string(),
        Box::new(Agg {
            name: "Blessed bracers",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_vambraces",
        }),
    );
    map.insert(
        "Saradomin chaps".to_string(),
        Box::new(Agg {
            name: "Blessed chaps",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_chaps",
        }),
    );
    map.insert(
        "Saradomin coif".to_string(),
        Box::new(Agg {
            name: "Blessed coif",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_coif",
        }),
    );
    map.insert(
        "Saradomin d'hide body".to_string(),
        Box::new(Agg {
            name: "Blessed d'hide body",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_body",
        }),
    );
    map.insert(
        "Saradomin d'hide boots".to_string(),
        Box::new(Agg {
            name: "Blessed d'hide boots",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_boots",
        }),
    );
    map.insert(
        "Saradomin d'hide shield".to_string(),
        Box::new(Agg {
            name: "Blessed d'hide shield",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessed_shield",
        }),
    );
}
