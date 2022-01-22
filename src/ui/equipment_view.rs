//! View the currently selected equipment set with a similar layout to the in-game equipment panel.

use data::EquipSlot;
use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::{attrs, div, img, style, C};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["equipment-view"],
        EquipSlot::into_enum_iter().map(|s| view_slot(model, s))
    ]
}

fn view_slot(model: &Model, slot: EquipSlot) -> Node<Msg> {
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

fn get_offsets(slot: EquipSlot) -> (u8, u8) {
    match slot {
        EquipSlot::Head => (84, 11),
        EquipSlot::Cape => (43, 50),
        EquipSlot::Neck => (84, 50),
        EquipSlot::Ammunition => (125, 50),
        EquipSlot::TwoHanded => (43, 129),
        EquipSlot::Weapon => (28, 89),
        EquipSlot::Body => (84, 89),
        EquipSlot::Shield => (140, 89),
        EquipSlot::Legs => (84, 129),
        EquipSlot::Hands => (28, 169),
        EquipSlot::Feet => (84, 169),
        EquipSlot::Ring => (140, 169),
    }
}
