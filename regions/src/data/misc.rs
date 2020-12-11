//! Items with drop sources in multiple regions or items which can only be obtained in Zeah.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

macro_rules! or {
    ($r1:expr, $($r:expr),*) => {
        {
            let mut builder = BoolExprBuilder::new();
            builder.var($r1);
            $(
                builder.var($r);
                builder.or();
            )*
            builder.finalize().unwrap()
        }
    }
}

/// Add requirements for the following items:
/// - [Dragon chainbody](https://oldschool.runescape.wiki/w/Dragon_chainbody)
/// - [Dragon 2h](https://oldschool.runescape.wiki/w/Dragon_2h_sword)
/// - [Ancient staff](https://oldschool.runescape.wiki/w/Ancient_staff)
/// - [Slayer drops](https://oldschool.runescape.wiki/w/Slayer_monsters)
/// - [Mystic robes](https://oldschool.runescape.wiki/w/Mystic_robes)
/// - [Ava's devices](https://oldschool.runescape.wiki/w/Ava%27s_device)
/// - [Ghostly robes](https://oldschool.runescape.wiki/w/Ghostly_robes)
///
/// Sets the following items to be unobtainable under trailblazer rules:
/// - [Dragon harpoon](https://oldschool.runescape.wiki/w/Dragon_harpoon)
/// - [Proselyte armour](https://oldschool.runescape.wiki/w/Proselyte_armour)
/// - [CoX Rewards](https://oldschool.runescape.wiki/w/Chambers_of_Xeric#Unique_drop_table)
/// - [Karuulm Slayer Dungeon Drops](https://oldschool.runescape.wiki/w/Karuulm_Slayer_Dungeon)
/// - [Xerician robes](https://oldschool.runescape.wiki/w/Xerician_robes)
/// - [Shayzien armour](https://oldschool.runescape.wiki/w/Shayzien_armour)
/// - [Rada's blessing](https://oldschool.runescape.wiki/w/Rada's_blessing)
pub fn add_items(map: &mut ExprMap) {
    map.insert(
        "Dragon chainbody".to_string(),
        or!(Region::Desert, Region::Kandarin),
    );
    map.insert(
        "Dragon 2h sword".to_string(),
        or!(Region::Wilderness, Region::Desert),
    );
    map.insert(
        "Ancient staff".to_string(),
        or!(Region::Desert, Region::Wilderness, Region::Kandarin),
    );

    map.insert("Xerician hat".to_string(), BoolExpr::new_false());
    map.insert("Xerician top".to_string(), BoolExpr::new_false());
    map.insert("Xerician robe".to_string(), BoolExpr::new_false());

    for i in 1..=5 {
        map.insert(format!("Shayzien boots ({})", i), BoolExpr::new_false());
        map.insert(format!("Shayzien gloves ({})", i), BoolExpr::new_false());
        map.insert(format!("Shayzien greaves ({})", i), BoolExpr::new_false());
        map.insert(format!("Shayzien helm ({})", i), BoolExpr::new_false());
        map.insert(format!("Shayzien platebody ({})", i), BoolExpr::new_false());
    }

    map.insert("Proselyte sallet".to_string(), BoolExpr::new_false());
    map.insert("Proselyte hauberk".to_string(), BoolExpr::new_false());
    map.insert("Proselyte cuisse".to_string(), BoolExpr::new_false());

    // Rada's blessing 1 has no bonuses
    map.insert("Rada's blessing 2".to_string(), BoolExpr::new_false());
    map.insert("Rada's blessing 3".to_string(), BoolExpr::new_false());
    map.insert("Rada's blessing 4".to_string(), BoolExpr::new_false());

    add_cox(map);
    add_karuulm(map);
    add_avas(map);
    add_mystic(map);
    add_slayer(map);
    add_ghostly(map);
}

fn add_karuulm(map: &mut ExprMap) {
    map.insert("Dragon sword".to_string(), BoolExpr::new_false());
    map.insert("Dragon harpoon".to_string(), BoolExpr::new_false());
    map.insert(
        "Dragon knife (Unpoisoned)".to_string(),
        BoolExpr::new_false(),
    );
    map.insert("Dragon thrownaxe".to_string(), BoolExpr::new_false());

    map.insert("Devout boots".to_string(), BoolExpr::new_false());
    map.insert("Boots of brimstone".to_string(), BoolExpr::new_false());

    map.insert("Brimstone ring".to_string(), BoolExpr::new_false());
    map.insert("Bonecrusher necklace".to_string(), BoolExpr::new_false());
    map.insert("Ferocious gloves".to_string(), BoolExpr::new_false());
    map.insert("Dragon hunter lance".to_string(), BoolExpr::new_false());
}

fn add_cox(map: &mut ExprMap) {
    map.insert("Twisted buckler".to_string(), BoolExpr::new_false());
    map.insert("Dragon hunter crossbow".to_string(), BoolExpr::new_false());
    map.insert("Dinh's bulwark".to_string(), BoolExpr::new_false());
    map.insert("Ancestral hat".to_string(), BoolExpr::new_false());
    map.insert("Ancestral robe top".to_string(), BoolExpr::new_false());
    map.insert("Ancestral robe bottom".to_string(), BoolExpr::new_false());
    map.insert("Dragon claws".to_string(), BoolExpr::new_false());
    map.insert("Elder maul".to_string(), BoolExpr::new_false());
    map.insert("Kodai wand".to_string(), BoolExpr::new_false());
    map.insert("Twisted bow".to_string(), BoolExpr::new_false());
}

fn add_avas(map: &mut ExprMap) {
    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Asgarnia);
    expr.var(Region::Morytania);
    expr.and();
    expr.var(Region::Fremennik);
    expr.or();
    let expr = expr.finalize().unwrap();

    map.insert("Ava's attractor".to_string(), expr.clone());
    map.insert("Ava's accumulator".to_string(), expr);
}

fn add_mystic(map: &mut ExprMap) {
    map.insert("Mystic hat".to_string(), BoolExpr::new(Region::Kandarin));
    map.insert(
        "Mystic robe top".to_string(),
        or!(Region::Kandarin, Region::Wilderness),
    );
    // Other pieces obtainable from implings
}

fn add_slayer(map: &mut ExprMap) {
    // Banshee
    map.insert(
        "Mystic gloves (dark)".to_string(),
        BoolExpr::new(Region::Morytania),
    );

    // Cockatrice
    map.insert(
        "Mystic boots (light)".to_string(),
        BoolExpr::new(Region::Fremennik),
    );

    // Mogre
    map.insert("Flippers".to_string(), BoolExpr::new(Region::Asgarnia));

    // Terror dog
    map.insert(
        "Granite helm".to_string(),
        or!(Region::Kandarin, Region::Morytania),
    );

    // Infernal mage
    map.insert(
        "Mystic boots (dark)".to_string(),
        BoolExpr::new(Region::Morytania),
    );
    map.insert(
        "Mystic hat (dark)".to_string(),
        BoolExpr::new(Region::Morytania),
    );

    // Brine rat
    map.insert("Brine sabre".to_string(), BoolExpr::new(Region::Fremennik));

    // Bloodveld
    map.insert(
        "Black boots".to_string(),
        or!(
            Region::Asgarnia,
            Region::Morytania,
            Region::Kandarin,
            Region::Tirannwn,
            Region::Wilderness
        ),
    );

    // Jelly
    map.insert(
        "Mithril boots".to_string(),
        BoolExpr::new(Region::Fremennik),
    );

    // Turoth
    map.insert(
        "Mystic robe bottom (light)".to_string(),
        BoolExpr::new(Region::Fremennik),
    );

    // Aberrant spectre
    map.insert(
        "Mystic robe bottom (dark)".to_string(),
        or!(Region::Kandarin, Region::Morytania),
    );

    // Kurask
    map.insert(
        "Mystic robe top (light)".to_string(),
        or!(Region::Fremennik, Region::Tirannwn),
    );
    map.insert(
        "Leaf-bladed sword".to_string(),
        or!(Region::Fremennik, Region::Tirannwn),
    );
    map.insert(
        "Leaf-bladed battleaxe".to_string(),
        or!(Region::Fremennik, Region::Tirannwn),
    );

    // Skeletal wyvern
    map.insert("Granite legs".to_string(), BoolExpr::new(Region::Asgarnia));

    // Gargoyle
    map.insert(
        "Mystic robe top (dark)".to_string(),
        BoolExpr::new(Region::Morytania),
    );
    map.insert(
        "Adamant boots".to_string(),
        BoolExpr::new(Region::Morytania),
    );

    // Nechryael
    map.insert(
        "Rune boots".to_string(),
        or!(Region::Morytania, Region::Tirannwn),
    );
}

fn add_ghostly(map: &mut ExprMap) {
    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Desert);
    expr.var(Region::Asgarnia);
    expr.var(Region::Kandarin);
    expr.var(Region::Wilderness);
    expr.and();
    expr.and();
    expr.and();
    let expr = expr.finalize().unwrap();

    map.insert("Ghostly boots".to_string(), expr.clone());
    map.insert("Ghostly cloak".to_string(), expr.clone());
    map.insert("Ghostly gloves".to_string(), expr.clone());
    map.insert("Ghostly hood".to_string(), expr.clone());
    map.insert("Ghostly robe (bottom)".to_string(), expr.clone());
    map.insert("Ghostly robe (top)".to_string(), expr);
}
