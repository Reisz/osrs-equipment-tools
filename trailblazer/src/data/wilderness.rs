//! Items mainly locked by the Wilderness.

use super::ExprMap;
use crate::{bool_expr::BoolExpr, vars::Region};

fn expr() -> BoolExpr<Region> {
    BoolExpr::new(Region::Wilderness)
}

/// Add requirements for the following items:
/// - [Dragon pick](https://oldschool.runescape.wiki/w/Dragon_pickaxe)
/// - [Malediction ward](https://oldschool.runescape.wiki/w/Malediction_ward)
/// - [Wilderness rings](https://oldschool.runescape.wiki/w/Wilderness_rings)
/// - [Dagon'hai robes](https://oldschool.runescape.wiki/w/Dagon%27hai_robes)
/// - [Elder chaos robes](https://oldschool.runescape.wiki/w/Elder_chaos_druid_robes)
/// - [Corp drops](https://oldschool.runescape.wiki/w/Corporeal_Beast)
/// - [Beacon ring](https://oldschool.runescape.wiki/w/Beacon_ring)
/// - [Wildeness sword](https://oldschool.runescape.wiki/w/Wilderness_sword)
pub fn add_items(map: &mut ExprMap) {
    map.insert("Dragon pickaxe".to_string(), expr());

    map.insert("Malediction ward".to_string(), expr());
    map.insert("Odium ward".to_string(), expr());

    map.insert("Ring of the gods".to_string(), expr());
    map.insert("Treasonous ring".to_string(), expr());
    map.insert("Tyrannical ring".to_string(), expr());

    map.insert("Dagon'hai hat".to_string(), expr());
    map.insert("Dagon'hai robe bottom".to_string(), expr());
    map.insert("Dagon'hai robe top".to_string(), expr());

    map.insert("Elder chaos hood".to_string(), expr());
    map.insert("Elder chaos robe".to_string(), expr());
    map.insert("Elder chaos top".to_string(), expr());

    map.insert("Spirit shield".to_string(), expr());
    map.insert("Blessed spirit shield".to_string(), expr());
    map.insert("Spectral spirit shield".to_string(), expr());
    map.insert("Arcane spirit shield".to_string(), expr());
    map.insert("Elysian spirit shield".to_string(), expr());

    map.insert("Wilderness sword 1".to_string(), expr());
    map.insert("Wilderness sword 2".to_string(), expr());
    map.insert("Wilderness sword 3".to_string(), expr());
    map.insert("Wilderness sword 4".to_string(), expr());
}
