//! User interface code.

mod equipment_view;
mod region_buttons;
mod sorting_preset_buttons;

use seed::{div, prelude::*};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    if model.is_loading() {
        div!["Loading..."]
    } else {
        div![
            region_buttons::view(&model.trailblazer),
            sorting_preset_buttons::view(),
            equipment_view::view(model)
        ]
    }
}
