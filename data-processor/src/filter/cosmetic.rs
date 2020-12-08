//! Filter out [cosmetic](https://oldschool.runescape.wiki/w/Ornament_kit) variants.

use std::collections::HashSet;

/// Filters out the following cosmetic variants:
/// - [Twisted ancestral](https://oldschool.runescape.wiki/w/Twisted_ancestral_colour_kit)
/// - [Infinity robe variants](https://oldschool.runescape.wiki/w/Infinity_robes)
/// - [Gilded](https://oldschool.runescape.wiki/w/Gilded_equipment)
/// - [Whip mix variants](https://oldschool.runescape.wiki/w/Whip_mix)
/// - [Mutagen variants](https://oldschool.runescape.wiki/w/Mutagen)
/// - [Enchanted robes](https://oldschool.runescape.wiki/w/Enchanted_robes)
/// - [Robes of darkness](https://oldschool.runescape.wiki/w/Robes_of_darkness)
/// - [Obsidian cape (r)](https://oldschool.runescape.wiki/w/Obsidian_cape_(r))
/// - [Slayer helmet](https://oldschool.runescape.wiki/w/Slayer_helmet#Upgrading)
/// - [Dragonstone armour](https://oldschool.runescape.wiki/w/Dragonstone_armour)
///     - Except gauntlets which offer unique stats and different requirements
/// - [Trailblazer tools](https://oldschool.runescape.wiki/w/Trailblazer_tool_ornament_kit)
pub fn add_names(set: &mut HashSet<String>) {
    set.insert("Twisted ancestral hat".to_string());
    set.insert("Twisted ancestral robe bottom".to_string());
    set.insert("Twisted ancestral robe top".to_string());

    set.insert("Light infinity bottoms".to_string());
    set.insert("Light infinity hat".to_string());
    set.insert("Light infinity top".to_string());

    set.insert("Dark infinity bottoms".to_string());
    set.insert("Dark infinity hat".to_string());
    set.insert("Dark infinity top".to_string());

    set.insert("Gilded boots".to_string());
    set.insert("Gilded chainbody".to_string());
    set.insert("Gilded full helm".to_string());
    set.insert("Gilded kiteshield".to_string());
    set.insert("Gilded med helm".to_string());
    set.insert("Gilded platebody".to_string());
    set.insert("Gilded platelegs".to_string());
    set.insert("Gilded plateskirt".to_string());
    set.insert("Gilded sq shield".to_string());

    set.insert("Gilded coif".to_string());
    set.insert("Gilded d'hide body".to_string());
    set.insert("Gilded d'hide chaps".to_string());
    set.insert("Gilded d'hide vambraces".to_string());

    set.insert("Gilded 2h sword".to_string());
    set.insert("Gilded axe".to_string());
    set.insert("Gilded hasta".to_string());
    set.insert("Gilded pickaxe".to_string());
    set.insert("Gilded scimitar".to_string());
    set.insert("Gilded spear".to_string());

    set.insert("Frozen abyssal whip".to_string());
    set.insert("Volcanic abyssal whip".to_string());

    set.insert("Magma helm".to_string());
    set.insert("Tanzanite helm".to_string());

    // Some region combinations do not have access to mystic hat or top
    #[cfg(not(feature = "trailblazer"))]
    {
        set.insert("Enchanted hat".to_string());
        set.insert("Enchanted top".to_string());
    }
    set.insert("Enchanted robe".to_string());

    set.insert("Hood of darkness".to_string());
    set.insert("Robe top of darkness".to_string());
    set.insert("Robe bottom of darkness".to_string());
    set.insert("Gloves of darkness".to_string());
    set.insert("Boots of darkness".to_string());

    set.insert("Obsidian cape (r)".to_string());

    set.insert("Black slayer helmet".to_string());
    set.insert("Green slayer helmet".to_string());
    set.insert("Red slayer helmet".to_string());
    set.insert("Purple slayer helmet".to_string());
    set.insert("Turquoise slayer helmet".to_string());
    set.insert("Hydra slayer helmet".to_string());
    set.insert("Twisted slayer helmet".to_string());

    set.insert("Dragonstone boots".to_string());
    set.insert("Dragonstone full helm".to_string());
    set.insert("Dragonstone platebody".to_string());
    set.insert("Dragonstone platelegs".to_string());

    set.insert("Trailblazer axe".to_string());
    set.insert("Trailblazer harpoon".to_string());
    set.insert("Trailblazer pickaxe".to_string());
}

/// Filters out the following cosmetic variants:
/// - [Ward upgrade kit upgrades](https://oldschool.runescape.wiki/w/Ward_upgrade_kit)
pub fn add_wiki_names(set: &mut HashSet<String>) {
    set.insert("Malediction ward (or)".to_string());
    set.insert("Odium ward (or)".to_string());
}
