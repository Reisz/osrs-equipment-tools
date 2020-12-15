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
/// - [Cape of skulls](https://oldschool.runescape.wiki/w/Cape_of_skulls)
/// - [Rain bow](https://oldschool.runescape.wiki/w/Rain_bow)
/// - [Slayer helmet](https://oldschool.runescape.wiki/w/Slayer_helmet#Upgrading)
/// - [Dragonstone armour](https://oldschool.runescape.wiki/w/Dragonstone_armour)
///     - Except gauntlets which offer unique stats and different requirements
/// - [Ardougne knight armour](https://oldschool.runescape.wiki/w/Ardougne_knight_armour)
/// - [Corrupted armour](https://oldschool.runescape.wiki/w/Corrupted_armour)
/// - [Clue hunter outfit](https://oldschool.runescape.wiki/w/Clue_hunter_outfit)
/// - [Ornate armour](https://oldschool.runescape.wiki/w/Ornate_armour)
/// - [Amulet of glory (t)](https://oldschool.runescape.wiki/w/Amulet_of_glory_(t)#Uncharged)
/// - [Spiked boots](https://oldschool.runescape.wiki/w/Spiked_boots )
/// - [Blacksmith's helm](https://oldschool.runescape.wiki/w/Blacksmith's_helm)
/// - [Smouldering stone upgrades](https://oldschool.runescape.wiki/w/Smouldering_stone)
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

    // Easy clue
    set.insert("Cape of skulls".to_string());
    set.insert("Rain bow".to_string());

    for i in &["", " (i)"] {
        set.insert(format!("Black slayer helmet{}", i));
        set.insert(format!("Green slayer helmet{}", i));
        set.insert(format!("Red slayer helmet{}", i));
        set.insert(format!("Purple slayer helmet{}", i));
        set.insert(format!("Turquoise slayer helmet{}", i));
        set.insert(format!("Hydra slayer helmet{}", i));
        set.insert(format!("Twisted slayer helmet{}", i));
    }

    set.insert("Dragonstone boots".to_string());
    set.insert("Dragonstone full helm".to_string());
    set.insert("Dragonstone platebody".to_string());
    set.insert("Dragonstone platelegs".to_string());

    set.insert("Ardougne knight helm".to_string());
    set.insert("Ardougne knight platebody".to_string());
    set.insert("Ardougne knight platelegs".to_string());

    set.insert("Corrupted helm".to_string());
    set.insert("Corrupted kiteshield".to_string());
    set.insert("Corrupted platebody".to_string());
    set.insert("Corrupted platelegs".to_string());
    set.insert("Corrupted plateskirt".to_string());

    set.insert("Helm of raedwald".to_string());
    set.insert("Clue hunter garb".to_string());
    set.insert("Clue hunter trousers".to_string());
    set.insert("Clue hunter gloves".to_string());
    set.insert("Clue hunter boots".to_string());
    set.insert("Clue hunter cloak".to_string());

    set.insert("Ornate helm".to_string());
    set.insert("Ornate top".to_string());
    set.insert("Ornate legs".to_string());
    set.insert("Ornate gloves".to_string());
    set.insert("Ornate boots".to_string());
    set.insert("Ornate cape".to_string());

    set.extend((1..=6).map(|i| format!("Amulet of glory (t{})", i)));

    set.insert("Spiked boots".to_string());

    set.insert("Blacksmith's helm".to_string());

    set.insert("Infernal axe".to_string());
    set.insert("Infernal harpoon".to_string());
    set.insert("Infernal pickaxe".to_string());
}

/// Filters out the following cosmetic variants:
/// - [Ward upgrade kit upgrades](https://oldschool.runescape.wiki/w/Ward_upgrade_kit)
/// - [Decorative armour](https://oldschool.runescape.wiki/w/Decorative_armour)
/// - [Trailblazer tools](https://oldschool.runescape.wiki/w/Trailblazer_tool_ornament_kit)
/// - [Crystal tools](https://oldschool.runescape.wiki/w/Crystal_tool)
/// - [Dark bow variants](https://oldschool.runescape.wiki/w/Dark_bow)
/// - [Rune scimitar ornaments](https://oldschool.runescape.wiki/w/Rune_scimitar_ornament_kit)
/// - [Upgraded dragon pickaxe](https://oldschool.runescape.wiki/w/Dragon_pickaxe_(upgraded))
pub fn add_wiki_names(set: &mut HashSet<String>) {
    set.insert("Malediction ward (or)".to_string());
    set.insert("Odium ward (or)".to_string());

    for (color, trouver) in &[("red", ""), ("white", ""), ("gold", " (Normal)")] {
        set.insert(format!("Decorative full helm ({}){}", color, trouver));
        set.insert(format!("Decorative helm ({}){}", color, trouver));
        set.insert(format!(
            "Decorative armour ({} platebody){}",
            color, trouver
        ));
        set.insert(format!(
            "Decorative armour ({} platelegs){}",
            color, trouver
        ));
        set.insert(format!(
            "Decorative armour ({} plateskirt){}",
            color, trouver
        ));
        set.insert(format!("Decorative boots ({}){}", color, trouver));
        set.insert(format!("Decorative shield ({}){}", color, trouver));
        set.insert(format!("Decorative sword ({}){}", color, trouver));
    }

    for or in &["", " (or)"] {
        set.insert(format!("Trailblazer axe{}", or));
        set.insert(format!("Trailblazer harpoon{}", or));
        set.insert(format!("Trailblazer pickaxe{}", or));
    }

    for active in &["Active", "Inactive"] {
        set.insert(format!("Crystal axe ({})", active));
        set.insert(format!("Crystal harpoon ({})", active));
        set.insert(format!("Crystal pickaxe ({})", active));
    }

    set.insert("Dark bow (Green)".to_string());
    set.insert("Dark bow (Blue)".to_string());
    set.insert("Dark bow (Yellow)".to_string());
    set.insert("Dark bow (White)".to_string());

    set.insert("Rune scimitar (saradomin)".to_string());
    set.insert("Rune scimitar (zamorak)".to_string());
    set.insert("Rune scimitar (guthix)".to_string());

    set.insert("Dragon pickaxe (upgraded)".to_string());
}
