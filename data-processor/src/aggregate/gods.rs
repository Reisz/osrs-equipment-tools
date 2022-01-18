//! Consolidate God-based equipment

use std::{collections::HashSet, hash::BuildHasher};

use super::{AggregationMap, Aggregator};
use crate::osrsbox::ItemProperties;

const FILTERED_GODS: &[&str] = &["Guthix", "Zamorak", "Armadyl", "Bandos", "Ancient"];

/// Add wiki names of items to filter.
pub fn add_filter_names<S: BuildHasher>(set: &mut HashSet<String, S>) {
    for god in FILTERED_GODS {
        set.insert(format!("{} full helm", god));
        set.insert(format!("{} kiteshield", god));
        set.insert(format!("{} platebody", god));
        set.insert(format!("{} platelegs", god));

        set.insert(format!("{} bracers", god));
        set.insert(format!("{} chaps", god));
        set.insert(format!("{} coif", god));
        set.insert(format!("{} d'hide body", god));
        set.insert(format!("{} d'hide boots", god));
        set.insert(format!("{} d'hide shield", god));

        set.insert(format!("{} cloak", god));
        set.insert(format!("{} mitre", god));
        set.insert(format!("{} robe legs", god));
        set.insert(format!("{} robe top", god));
        set.insert(format!("{} stole", god));
        set.insert(format!("{} crozier", god));

        set.insert(format!("Damaged book ({})", god));
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
#[allow(clippy::too_many_lines)]
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

    map.insert(
        "Damaged book (Saradomin)".to_string(),
        Box::new(Agg {
            name: "Damaged book",
            wiki_url: "https://oldschool.runescape.wiki/w/Damaged_book",
        }),
    );
}
