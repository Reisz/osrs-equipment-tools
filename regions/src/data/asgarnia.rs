//! Items mainly locked by Asgarnia.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Asgarnia)
}

fn expr_and(region: Region) -> BoolExpr<Region> {
    let mut builder = BoolExprBuilder::new();
    builder.var(Region::Asgarnia);
    builder.var(region);
    builder.and();
    builder.finalize().unwrap()
}

/// Add requirements for the following items:
/// - [Boot upgrades](https://oldschool.runescape.wiki/w/Cerberus)
/// - [God Wars Dungeon](https://oldschool.runescape.wiki/w/God_Wars_Dungeon)
/// - [Defenders](https://oldschool.runescape.wiki/w/Defender)
/// - [Initiate armour](https://oldschool.runescape.wiki/w/Initiate_armour)
/// - [Falador shield](https://oldschool.runescape.wiki/w/Falador_shield)
/// - Skill capes
///     - [Attack cape](https://oldschool.runescape.wiki/w/Attack_cape)
///     - [Crafting cape](https://oldschool.runescape.wiki/w/Crafting_cape)
///     - [Herblore cape](https://oldschool.runescape.wiki/w/Herblore_cape)
///     - [Mining cape](https://oldschool.runescape.wiki/w/Mining_cape)
///     - [Prayer cape](https://oldschool.runescape.wiki/w/Prayer_cape)
///     - [Smithing cape](https://oldschool.runescape.wiki/w/Smithing_cape)
///     - [Strength cape](https://oldschool.runescape.wiki/w/Strength_cape)
///     - [Thieving cape](https://oldschool.runescape.wiki/w/Thieving_cape)
/// - [Max capes](https://oldschool.runescape.wiki/w/Max_cape)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Falador shield 1".to_string(), expr());
    map.insert("Falador shield 2".to_string(), expr());
    map.insert("Falador shield 3".to_string(), expr());
    map.insert("Falador shield 4".to_string(), expr());

    add_cerb(map);
    add_gwd(map);
    add_defenders(map);
    add_initiate(map);
    add_capes(map);
    add_max(map);
}

fn add_cerb(map: &mut ExprMap) {
    map.insert("Primordial boots".to_string(), expr());
    map.insert("Pegasian boots".to_string(), expr());

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Desert);
    expr.var(Region::Asgarnia);
    expr.and();
    let expr = expr.finalize().unwrap();

    map.insert("Eternal boots".to_string(), expr);
}

fn add_gwd(map: &mut ExprMap) {
    map.insert("Dragon boots".to_string(), expr());

    map.insert("Armadyl helmet".to_string(), expr());
    map.insert("Armadyl chestplate".to_string(), expr());
    map.insert("Armadyl chainskirt".to_string(), expr());
    map.insert("Armadyl godsword".to_string(), expr());

    map.insert("Bandos chestplate".to_string(), expr());
    map.insert("Bandos tassets".to_string(), expr());
    map.insert("Bandos boots".to_string(), expr());
    map.insert("Bandos godsword".to_string(), expr());

    map.insert("Saradomin sword".to_string(), expr());
    map.insert("Armadyl crossbow".to_string(), expr());
    map.insert("Saradomin godsword".to_string(), expr());

    map.insert("Steam battlestaff".to_string(), expr());
    map.insert("Zamorakian spear".to_string(), expr());
    map.insert("Zamorak godsword".to_string(), expr());
}

fn add_defenders(map: &mut ExprMap) {
    map.insert("Bronze defender (Normal)".to_string(), expr());
    map.insert("Iron defender (Normal)".to_string(), expr());
    map.insert("Steel defender (Normal)".to_string(), expr());
    map.insert("Black defender (Normal)".to_string(), expr());
    map.insert("Mithril defender (Normal)".to_string(), expr());
    map.insert("Adamant defender (Normal)".to_string(), expr());
    map.insert("Rune defender (Normal)".to_string(), expr());
    map.insert("Dragon defender (Normal)".to_string(), expr());
}

fn add_initiate(map: &mut ExprMap) {
    map.insert("Initiate sallet".to_string(), expr());
    map.insert("Initiate hauberk".to_string(), expr());
    map.insert("Initiate cuisse".to_string(), expr());
}

fn add_capes(map: &mut ExprMap) {
    map.insert("Attack cape (Untrimmed)".to_string(), expr());
    map.insert("Attack cape (Trimmed)".to_string(), expr());

    map.insert("Crafting cape (Untrimmed)".to_string(), expr());
    map.insert("Crafting cape (Trimmed)".to_string(), expr());

    map.insert("Herblore cape (Untrimmed)".to_string(), expr());
    map.insert("Herblore cape (Trimmed)".to_string(), expr());

    map.insert("Mining cape (Untrimmed)".to_string(), expr());
    map.insert("Mining cape (Trimmed)".to_string(), expr());

    map.insert("Prayer cape (Untrimmed)".to_string(), expr());
    map.insert("Prayer cape (Trimmed)".to_string(), expr());

    map.insert("Smithing cape (Untrimmed)".to_string(), expr());
    map.insert("Smithing cape (Trimmed)".to_string(), expr());

    map.insert("Strength cape (Untrimmed)".to_string(), expr());
    map.insert("Strength cape (Trimmed)".to_string(), expr());

    map.insert("Thieving cape (Untrimmed)".to_string(), expr());
    map.insert("Thieving cape (Trimmed)".to_string(), expr());
}

fn add_max(map: &mut ExprMap) {
    map.insert("Max cape".to_string(), expr());
    map.insert("Fire max cape (Normal)".to_string(), expr());
    map.insert("Infernal max cape (Normal)".to_string(), expr());

    map.insert("Ardougne max cape".to_string(), expr_and(Region::Kandarin));
    map.insert(
        "Assembler max cape (Normal)".to_string(),
        expr_and(Region::Fremennik),
    );

    map.insert(
        "Saradomin max cape".to_string(),
        expr_and(Region::Wilderness),
    );
    map.insert("Zamorak max cape".to_string(), expr_and(Region::Wilderness));
    map.insert("Guthix max cape".to_string(), expr_and(Region::Wilderness));

    map.insert(
        "Imbued saradomin max cape (Normal)".to_string(),
        expr_and(Region::Wilderness),
    );
    map.insert(
        "Imbued zamorak max cape (Normal)".to_string(),
        expr_and(Region::Wilderness),
    );
    map.insert(
        "Imbued guthix max cape (Normal)".to_string(),
        expr_and(Region::Wilderness),
    );

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Asgarnia);
    expr.var(Region::Fremennik);
    expr.var(Region::Morytania);
    expr.or();
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Accumulator max cape".to_string(), expr);

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Asgarnia);
    expr.var(Region::Fremennik);
    expr.var(Region::Kandarin);
    expr.and();
    expr.and();
    let expr = expr.finalize().unwrap();
    map.insert("Mythical max cape".to_string(), expr);
}
