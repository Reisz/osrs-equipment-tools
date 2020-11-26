//! Filter out [cosmetic](https://oldschool.runescape.wiki/w/Ornament_kit) variants.

use std::collections::HashSet;

/// Filters out the following cosmetic variants:
/// - [Twisted ancestral](https://oldschool.runescape.wiki/w/Twisted_ancestral_colour_kit)
/// - [Infinity robe variants](https://oldschool.runescape.wiki/w/Infinity_robes)
/// - [Gilded](https://oldschool.runescape.wiki/w/Gilded_equipment)
/// - [Whip mix variants](https://oldschool.runescape.wiki/w/Whip_mix)
/// - [Mutagen variants](https://oldschool.runescape.wiki/w/Mutagen)
/// - [Obsidian cape (r)](https://oldschool.runescape.wiki/w/Obsidian_cape_(r))
pub fn add_cosmetics(list: &mut HashSet<String>) {
    list.insert("Twisted ancestral hat".to_string());
    list.insert("Twisted ancestral robe bottom".to_string());
    list.insert("Twisted ancestral robe top".to_string());

    list.insert("Light infinity bottoms".to_string());
    list.insert("Light infinity hat".to_string());
    list.insert("Light infinity top".to_string());

    list.insert("Dark infinity bottoms".to_string());
    list.insert("Dark infinity hat".to_string());
    list.insert("Dark infinity top".to_string());

    list.insert("Gilded boots".to_string());
    list.insert("Gilded chainbody".to_string());
    list.insert("Gilded full helm".to_string());
    list.insert("Gilded kiteshield".to_string());
    list.insert("Gilded med helm".to_string());
    list.insert("Gilded platebody".to_string());
    list.insert("Gilded platelegs".to_string());
    list.insert("Gilded plateskirt".to_string());
    list.insert("Gilded sq shield".to_string());

    list.insert("Gilded coif".to_string());
    list.insert("Gilded d'hide body".to_string());
    list.insert("Gilded d'hide chaps".to_string());
    list.insert("Gilded d'hide vambraces".to_string());

    list.insert("Gilded 2h sword".to_string());
    list.insert("Gilded axe".to_string());
    list.insert("Gilded hasta".to_string());
    list.insert("Gilded pickaxe".to_string());
    list.insert("Gilded scimitar".to_string());
    list.insert("Gilded spear".to_string());

    list.insert("Frozen abyssal whip".to_string());
    list.insert("Volcanic abyssal whip".to_string());

    list.insert("Magma helm".to_string());
    list.insert("Tanzanite helm".to_string());

    list.insert("Obsidian cape (r)".to_string());
}
