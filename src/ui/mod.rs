//! User interface code.

mod equipment_view;

use seed::{div, prelude::*};

use crate::model::{Model, Msg};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    if model.is_loading() {
        div!["Loading..."]
    } else {
        equipment_view::view(model)
    }
}
