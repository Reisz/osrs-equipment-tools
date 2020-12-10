//! View a sorted list of the available items for one slot.

use data::Item;
use seed::prelude::*;
use seed::*;

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
        td![item.stats.attack.stab.to_string()],
        td![item.stats.attack.slash.to_string()],
        td![item.stats.attack.crush.to_string()],
        td![item.stats.attack.magic.to_string()],
        td![item.stats.attack.ranged.to_string()],
        td![item.stats.defence.stab.to_string()],
        td![item.stats.defence.slash.to_string()],
        td![item.stats.defence.crush.to_string()],
        td![item.stats.defence.magic.to_string()],
        td![item.stats.defence.ranged.to_string()],
        td![item.stats.melee_strength.to_string()],
        td![item.stats.ranged_strength.to_string()],
        td![format!("{}%", item.stats.magic_damage)],
        td![item.stats.prayer.to_string()],
    ]
}
