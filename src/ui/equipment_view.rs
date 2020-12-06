//! View the currently selected equipment set with a similar layout to the in-game equipment panel.

use data::Slot;
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::*;

use crate::{event::Msg, model::Model};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["equipment-view"],
        Slot::into_enum_iter().map(|s| view_slot(model, s))
    ]
}

fn view_slot(model: &Model, slot: Slot) -> Node<Msg> {
    let (left, top) = get_offsets(slot);

    let left = format!("{}px", left);
    let top = format!("{}px", top);

    if let Some(item) = model.data()[slot].get(0) {
        let icon = format!("data:image/png;base64,{}", item.icon_data);

        div![
            C!["equipment equipment-blank"],
            style!["left" => left, "top" => top],
            div![
                C!["equipment-plinkp"],
                a![
                    attrs![At::Href => item.wiki_url, At::Title => item.name],
                    img![attrs![At::Src => icon]],
                ]
            ],
        ]
    } else {
        Node::Empty
    }
}

fn get_offsets(slot: Slot) -> (u8, u8) {
    match slot {
        Slot::Head => (84, 11),
        Slot::Cape => (43, 50),
        Slot::Neck => (84, 50),
        Slot::Ammunition => (125, 50),
        Slot::TwoHanded | Slot::Weapon => (28, 89),
        Slot::Body => (84, 89),
        Slot::Shield => (140, 89),
        Slot::Legs => (84, 129),
        Slot::Hands => (28, 169),
        Slot::Feet => (84, 169),
        Slot::Ring => (140, 169),
    }
}
