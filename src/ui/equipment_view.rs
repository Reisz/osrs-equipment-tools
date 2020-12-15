//! View the currently selected equipment set with a similar layout to the in-game equipment panel.

use data::Slot;
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::*;

use crate::model::{Model, Msg};

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

    if let Some(item) = model.get_item(slot, 0) {
        let icon = format!("data:image/bmp;base64,{}", base64::encode(&item.icon_data));

        div![
            ev(Ev::Click, move |_| Msg::ChangeList(slot)),
            C!["equipment equipment-blank"],
            style!["left" => left, "top" => top],
            img![attrs![At::Src => icon, At::Title => item.name]],
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
        Slot::TwoHanded => (43, 129),
        Slot::Weapon => (28, 89),
        Slot::Body => (84, 89),
        Slot::Shield => (140, 89),
        Slot::Legs => (84, 129),
        Slot::Hands => (28, 169),
        Slot::Feet => (84, 169),
        Slot::Ring => (140, 169),
    }
}
