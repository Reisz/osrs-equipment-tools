//! View the currently selected equipment set with a similar layout to the in-game equipment panel.

use data::{Slot, Stats};
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::{div, h3, table, td, tr};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    let mut stats = Stats::default();
    for slot in Slot::into_enum_iter() {
        stats += &model.get_item(slot, 0).unwrap().stats;
    }

    div![
        h3!["Attack Bonuses"],
        table![
            tr![td!["Stab"], td![stats.attack.stab.to_string()]],
            tr![td!["Slash"], td![stats.attack.slash.to_string()]],
            tr![td!["Stab"], td![stats.attack.stab.to_string()]],
            tr![td!["Magic"], td![stats.attack.magic.to_string()]],
            tr![td!["Ranged"], td![stats.attack.ranged.to_string()]],
        ],
        h3!["Defence Bonuses"],
        table![
            tr![td!["Stab"], td![stats.defence.stab.to_string()]],
            tr![td!["Slash"], td![stats.defence.slash.to_string()]],
            tr![td!["Stab"], td![stats.defence.stab.to_string()]],
            tr![td!["Magic"], td![stats.defence.magic.to_string()]],
            tr![td!["Ranged"], td![stats.defence.ranged.to_string()]],
        ],
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
