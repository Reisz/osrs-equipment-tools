//! Consolidate [Vestment Robes](https://oldschool.runescape.wiki/w/Vestment_robes)

use std::collections::HashSet;

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const FILTERED_GODS: &[&str] = &["Guthix", "Zamorak", "Armadyl", "Bandos", "Ancient"];

/// Add wiki names of items to filter.
pub fn add_filter_names(set: &mut HashSet<String>) {
    for god in FILTERED_GODS {
        set.insert(format!("{} cloak", god));
        set.insert(format!("{} mitre", god));
        set.insert(format!("{} robe legs", god));
        set.insert(format!("{} robe top", god));
        set.insert(format!("{} stole", god));
        set.insert(format!("{} crozier", god));
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
        "Saradomin cloak".to_string(),
        Box::new(Agg {
            name: "Vestment cloak",
            wiki_url: "https://oldschool.runescape.wiki/w/Vestment_cloak",
        }),
    );
    map.insert(
        "Saradomin mitre".to_string(),
        Box::new(Agg {
            name: "Mitre",
            wiki_url: "https://oldschool.runescape.wiki/w/Mitre",
        }),
    );
    map.insert(
        "Saradomin robe legs".to_string(),
        Box::new(Agg {
            name: "Vestment robe legs",
            wiki_url: "https://oldschool.runescape.wiki/w/Vestment_robe_legs",
        }),
    );
    map.insert(
        "Saradomin robe top".to_string(),
        Box::new(Agg {
            name: "Vestment robe top",
            wiki_url: "https://oldschool.runescape.wiki/w/Vestment_robe_top",
        }),
    );
    map.insert(
        "Saradomin stole".to_string(),
        Box::new(Agg {
            name: "Stole",
            wiki_url: "https://oldschool.runescape.wiki/w/Stole",
        }),
    );
    map.insert(
        "Saradomin crozier".to_string(),
        Box::new(Agg {
            name: "Crozier",
            wiki_url: "https://oldschool.runescape.wiki/w/Crozier",
        }),
    );
}
