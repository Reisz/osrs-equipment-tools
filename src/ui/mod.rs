//! User interface code.

mod equipment_view;
#[cfg(feature = "trailblazer")]
mod region_buttons;
mod sorting_preset_buttons;
mod stats_view;

use seed::{div, prelude::*};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
#[cfg(not(feature = "trailblazer"))]
pub fn view(model: &Model) -> Node<Msg> {
    if model.is_loading() {
        div!["Loading..."]
    } else {
        div![
            sorting_preset_buttons::view(),
            equipment_view::view(model),
            stats_view::view(model),
        ]
    }
}

/// Create the DOM according to the [`Model`].
#[cfg(feature = "trailblazer")]
pub fn view(model: &Model) -> Node<Msg> {
    if model.is_loading() {
        div!["Loading..."]
    } else {
        div![
            region_buttons::view(&model.trailblazer),
            sorting_preset_buttons::view(),
            equipment_view::view(model),
            stats_view::view(model),
        ]
    }
}
