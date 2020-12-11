//! Consolidate [Halos](https://oldschool.runescape.wiki/w/Halo) and
//! [Blessings](https://oldschool.runescape.wiki/w/Blessing)

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

    set.insert("Unholy blessing".to_string());
    set.insert("Peaceful blessing".to_string());
    set.insert("Honourable blessing".to_string());
    set.insert("War blessing".to_string());
    set.insert("Ancient blessing".to_string());

    set.insert("Zamorak banner".to_string());
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
        "Saradomin halo (Normal)".to_string(),
        Box::new(Agg {
            name: "Halo",
            wiki_url: "https://oldschool.runescape.wiki/w/Halo",
        }),
    );

    map.insert(
        "Holy blessing".to_string(),
        Box::new(Agg {
            name: "Blessing",
            wiki_url: "https://oldschool.runescape.wiki/w/Blessing",
        }),
    );

    map.insert(
        "Saradomin banner".to_string(),
        Box::new(Agg {
            name: "Castlewars banner",
            wiki_url: "https://oldschool.runescape.wiki/w/Saradomin_banner",
        }),
    );
}
