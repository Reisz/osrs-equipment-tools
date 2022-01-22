//! View a sorted list of the available items for one slot.

use data::{DamageType, Item};
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::{a, attrs, img, table, td, tr};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    if let Some(slot) = model.list {
        table![model.iter(slot).map(view_item)]
    } else {
        Node::Empty
    }
}

fn view_item(item: &Item) -> Node<Msg> {
    let icon = format!("data:image/bmp;base64,{}", base64::encode(&item.icon_data));

    tr![
        td![a![
            attrs![At::Href => item.wiki_url, At::Title => item.name],
            img![attrs![At::Src => icon]],
        ]],
        td![a![
            attrs![At::Href => item.wiki_url, At::Title => item.name],
            &item.name,
        ],],
        DamageType::into_enum_iter()
            .map(|damage_type| td![item.combat_stats.attack[damage_type].to_string()]),
        DamageType::into_enum_iter()
            .map(|damage_type| td![item.combat_stats.defence[damage_type].to_string()]),
        td![item.combat_stats.melee_strength.to_string()],
        td![item.combat_stats.ranged_strength.to_string()],
        td![format!("{}%", item.combat_stats.magic_damage)],
        td![item.combat_stats.prayer.to_string()],
    ]
}
