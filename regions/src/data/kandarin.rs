//! Items mainly locked by Kandarin.

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::Region,
};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Kandarin)
}

fn expr_and(region: Region) -> BoolExpr<Region> {
    let mut builder = BoolExprBuilder::new();
    builder.var(Region::Kandarin);
    builder.var(region);
    builder.and();
    builder.finalize().unwrap()
}

/// Add requirements for the following items:
/// - [Imbues](https://oldschool.runescape.wiki/w/Nightmare_Zone#Upgrades)
/// - [MM II Rewards](https://oldschool.runescape.wiki/w/Monkey_Madness_II#Rewards)
///     - [Zenyte jewellery](https://oldschool.runescape.wiki/w/Zenyte)
///     - [Ballistae](https://oldschool.runescape.wiki/w/Ballista)
/// - Slayer bosses
///     - [Kraken drops](https://oldschool.runescape.wiki/w/Kraken#Kraken)
///     - [Thermy drops](https://oldschool.runescape.wiki/w/Thermonuclear_smoke_devil)
///     - [Dragon full helm](https://oldschool.runescape.wiki/w/Dragon_full_helm)
/// - [BA Rewards](https://oldschool.runescape.wiki/w/Barbarian_Assault/Rewards#Armour)
/// - Achievement diaries
///     - [Ardougne cloak](https://oldschool.runescape.wiki/w/Ardougne_cloak)
///     - [Kandarin headgear](https://oldschool.runescape.wiki/w/Kandarin_headgear)
///     - [Western banner](https://oldschool.runescape.wiki/w/Western_banner)
/// - Skill capes
///     - [Firemaking Cape](https://oldschool.runescape.wiki/w/Firemaking_cape)
///     - [Fishing Cape](https://oldschool.runescape.wiki/w/Fishing_cape)
///     - [Fletching Cape](https://oldschool.runescape.wiki/w/Fletching_cape)
///     - [Hunter Cape](https://oldschool.runescape.wiki/w/Hunter_cape)
///     - [Magic Cape](https://oldschool.runescape.wiki/w/Magic_cape)
///     - [Ranging Cape](https://oldschool.runescape.wiki/w/Rangingr_cape)
pub fn add_items(map: &mut ExprMap) {
    add_imbues(map);
    add_gorillas(map);
    add_slayer(map);
    add_ba(map);
    add_diaries(map);
    add_capes(map);
}

fn add_imbues(map: &mut ExprMap) {
    map.insert(
        "Black mask (i) (Uncharged)".to_string(),
        expr_and(Region::Morytania),
    );
    map.insert("Slayer helmet (i)".to_string(), expr_and(Region::Morytania));
    map.insert("Salve amulet(i)".to_string(), expr_and(Region::Morytania));
    map.insert("Salve amulet(ei)".to_string(), expr_and(Region::Morytania));
    map.insert("Ring of suffering (i) (Uncharged)".to_string(), expr());
    map.insert(
        "Berserker ring (i)".to_string(),
        expr_and(Region::Fremennik),
    );
    map.insert("Warrior ring (i)".to_string(), expr_and(Region::Fremennik));
    map.insert("Archers ring (i)".to_string(), expr_and(Region::Fremennik));
    map.insert("Seers ring (i)".to_string(), expr_and(Region::Fremennik));
    map.insert(
        "Tyrannical ring (i)".to_string(),
        expr_and(Region::Wilderness),
    );
    map.insert(
        "Treasonous ring (i)".to_string(),
        expr_and(Region::Wilderness),
    );
    map.insert("Granite ring (i)".to_string(), expr_and(Region::Morytania));

    let mut expr = BoolExprBuilder::new();
    expr.var(Region::Wilderness);
    expr.var(Region::Kandarin);
    expr.var(Region::Morytania);
    expr.and();
    expr.and();
    let expr = expr.finalize().unwrap();

    map.insert("Ring of the gods (i)".to_string(), expr);
}

fn add_gorillas(map: &mut ExprMap) {
    map.insert("Ring of suffering (Uncharged)".to_string(), expr());
    map.insert("Necklace of anguish".to_string(), expr());
    map.insert("Tormented bracelet".to_string(), expr());
    map.insert("Amulet of torture".to_string(), expr());

    map.insert("Light ballista".to_string(), expr());
    map.insert("Heavy ballista".to_string(), expr());
}

fn add_slayer(map: &mut ExprMap) {
    map.insert(
        "Trident of the seas (Partially charged)".to_string(),
        expr(),
    );
    map.insert("Abyssal tentacle".to_string(), expr());

    map.insert("Occult necklace".to_string(), expr());
    map.insert("Smoke battlestaff".to_string(), expr());

    map.insert("Dragon full helm".to_string(), expr());
}

fn add_ba(map: &mut ExprMap) {
    map.insert("Fighter hat (Normal)".to_string(), expr());
    map.insert("Ranger hat (Normal)".to_string(), expr());
    map.insert("Healer hat (Normal)".to_string(), expr());
    map.insert("Runner hat (Normal)".to_string(), expr());

    map.insert("Fighter torso (Normal)".to_string(), expr());
    map.insert("Penance skirt (Normal)".to_string(), expr());
    map.insert("Runner boots".to_string(), expr());
    map.insert("Penance gloves".to_string(), expr());
    map.insert("Granite body".to_string(), expr());
}

fn add_diaries(map: &mut ExprMap) {
    map.insert("Ardougne cloak 1".to_string(), expr());
    map.insert("Ardougne cloak 2".to_string(), expr());
    map.insert("Ardougne cloak 3".to_string(), expr());
    map.insert("Ardougne cloak 4".to_string(), expr());

    map.insert("Kandarin headgear 1".to_string(), expr());
    map.insert("Kandarin headgear 2".to_string(), expr());
    map.insert("Kandarin headgear 3".to_string(), expr());
    map.insert("Kandarin headgear 4".to_string(), expr());

    map.insert("Western banner 1".to_string(), expr());
    map.insert("Western banner 2".to_string(), expr());
    map.insert("Western banner 3".to_string(), expr_and(Region::Tirannwn));
    map.insert("Western banner 4".to_string(), expr_and(Region::Tirannwn));
}

fn add_capes(map: &mut ExprMap) {
    map.insert("Firemaking cape (Untrimmed)".to_string(), expr());
    map.insert("Firemaking cape (Trimmed)".to_string(), expr());

    map.insert("Fishing cape (Untrimmed)".to_string(), expr());
    map.insert("Fishing cape (Trimmed)".to_string(), expr());

    map.insert("Fletching cape (Untrimmed)".to_string(), expr());
    map.insert("Fletching cape (Trimmed)".to_string(), expr());

    map.insert("Hunter cape (Untrimmed)".to_string(), expr());
    map.insert("Hunter cape (Trimmed)".to_string(), expr());

    map.insert("Magic cape (Untrimmed)".to_string(), expr());
    map.insert("Magic cape (Trimmed)".to_string(), expr());

    map.insert("Ranging cape (Untrimmed)".to_string(), expr());
    map.insert("Ranging cape (Trimmed)".to_string(), expr());
}
