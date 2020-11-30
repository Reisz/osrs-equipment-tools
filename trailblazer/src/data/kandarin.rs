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
pub fn add_items(map: &mut ExprMap) {
    add_imbues(map);
    add_gorillas(map);
    add_slayer(map);
    add_ba(map);
}

fn add_imbues(map: &mut ExprMap) {
    map.insert("Black mask (i)".to_string(), expr_and(Region::Morytania));
    map.insert("Slayer helmet (i)".to_string(), expr_and(Region::Morytania));
    map.insert("Salve amulet(i)".to_string(), expr_and(Region::Morytania));
    map.insert("Salve amulet(ei)".to_string(), expr_and(Region::Morytania));
    map.insert("Ring of suffering (i)".to_string(), expr());
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
    map.insert("Ring of suffering".to_string(), expr());
    map.insert("Necklace of anguish".to_string(), expr());
    map.insert("Tormented bracelet".to_string(), expr());
    map.insert("Amulet of torture".to_string(), expr());

    map.insert("Light ballista".to_string(), expr());
    map.insert("Heavy ballista".to_string(), expr());
}

fn add_slayer(map: &mut ExprMap) {
    map.insert("Trident of the seas".to_string(), expr());
    map.insert("Abyssal tentacle".to_string(), expr());

    map.insert("Occult necklace".to_string(), expr());
    map.insert("Smoke battlestaff".to_string(), expr());

    map.insert("Dragon full helm".to_string(), expr());
}

fn add_ba(map: &mut ExprMap) {
    map.insert("Fighter hat".to_string(), expr());
    map.insert("Ranger hat".to_string(), expr());
    map.insert("Healer hat".to_string(), expr());
    map.insert("Runner hat".to_string(), expr());

    map.insert("Fighter torso".to_string(), expr());
    map.insert("Penance skirt".to_string(), expr());
    map.insert("Runner boots".to_string(), expr());
    map.insert("Penance gloves".to_string(), expr());
    map.insert("Granite body".to_string(), expr());
}
