//! View the currently selected equipment set with a similar layout to the in-game equipment panel.

use data::{CombatStats, DamageType, EquipSlot};
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::{div, h3, table, td, tr};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    let mut stats = CombatStats::default();
    for slot in EquipSlot::into_enum_iter() {
        stats += &model.get_item(slot, 0).unwrap().combat_stats;
    }

    div![
        h3!["Attack Bonuses"],
        table![DamageType::into_enum_iter().map(|damage_type| tr![
            td![format!("{damage_type}")],
            td![stats.attack[damage_type].to_string()]
        ])],
        h3!["Defence Bonuses"],
        table![DamageType::into_enum_iter().map(|damage_type| tr![
            td![format!("{damage_type}")],
            td![stats.defence[damage_type].to_string()]
        ])],
        h3!["Other Bonuses"],
        table![
            tr![td!["Melee Strength"], td![stats.melee_strength.to_string()]],
            tr![
                td!["Ranged Strength"],
                td![stats.ranged_strength.to_string()]
            ],
            tr![td!["Magic Damage"], td![format!("{}%", stats.magic_damage)]],
            tr![td!["Prayer"], td![stats.prayer.to_string()]],
        ],
    ]
}
