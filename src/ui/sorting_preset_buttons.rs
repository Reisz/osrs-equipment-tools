//! Displays buttons to allow applying sorting presets

use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::*;

use crate::model::{
    sorting::{SortingMsg, SortingPreset},
    Msg,
};

/// Create the DOM according to the [`Model`].
pub fn view() -> Node<Msg> {
    div![SortingPreset::into_enum_iter().map(|r| view_button(r))]
}

fn view_button(preset: SortingPreset) -> Node<Msg> {
    span![
        C!["button"],
        ev(Ev::Click, move |_| Msg::Sorting(SortingMsg::ApplyPreset(
            preset
        ))),
        format!("{:?}", preset)
    ]
}
