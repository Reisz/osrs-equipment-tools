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
    ]
}
