//! Displays buttons to allow applying sorting presets

use enum_iterator::IntoEnumIterator;
use seed::prelude::*;
use seed::{div, span, C};

use crate::model::{
    sorting::{Msg as SortingMsg, Preset},
    Msg,
};

/// Create the DOM according to the [`Model`].
pub fn view() -> Node<Msg> {
    div![Preset::into_enum_iter().map(view_button)]
}

fn view_button(preset: Preset) -> Node<Msg> {
    span![
        C!["button"],
        ev(Ev::Click, move |_| Msg::Sorting(SortingMsg::ApplyPreset(
            preset
        ))),
        format!("{:?}", preset)
    ]
}
