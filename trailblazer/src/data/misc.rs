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
/// - [Leaf-bladed sword](https://oldschool.runescape.wiki/w/Leaf-bladed_sword)
/// - [Leaf-bladed battleaxe](https://oldschool.runescape.wiki/w/Leaf-bladed_battleaxe)
///
/// Sets the following items to be unobtainable under trailblazer rules:
/// - [Dragon harpoon](https://oldschool.runescape.wiki/w/Dragon_harpoon)
/// - [Proselyte armour](https://oldschool.runescape.wiki/w/Proselyte_armour)
/// - [CoX Rewards](https://oldschool.runescape.wiki/w/Chambers_of_Xeric#Unique_drop_table)
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

    map.insert(
        "Leaf-bladed sword".to_string(),
        or!(Region::Fremennik, Region::Tirannwn),
    );
    map.insert(
        "Leaf-bladed battleaxe".to_string(),
        or!(Region::Fremennik, Region::Tirannwn),
    );

    map.insert("Dragon harpoon".to_string(), BoolExpr::new_false());

    map.insert("Proselyte sallet".to_string(), BoolExpr::new_false());
    map.insert("Proselyte hauberk".to_string(), BoolExpr::new_false());
    map.insert("Proselyte tasset".to_string(), BoolExpr::new_false());
    map.insert("Proselyte cuisse".to_string(), BoolExpr::new_false());

    add_cox(map);
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
