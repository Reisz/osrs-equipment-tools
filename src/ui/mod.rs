//! User interface code.

use seed::{div, prelude::*};

use crate::{
    event::Msg,
    model::{Data, Model},
};

/// Create the DOM according to the [`Model`].
pub fn view(model: &Model) -> Node<Msg> {
    match &model.data {
        None => div!["Loading..."],
        Some(Data::Loading(count, total)) => {
            div![format!("Loading... ({}/{})", count, total)]
        }
        Some(Data::Done(_)) => div!["Done."],
    }
}
