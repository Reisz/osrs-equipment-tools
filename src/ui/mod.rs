//! User interface code.

use seed::{div, prelude::*};

use crate::{event::Msg, model::Model};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    match &model.data {
        None => div!["Loading..."],
        Some(_) => div!["Done."],
    }
}
