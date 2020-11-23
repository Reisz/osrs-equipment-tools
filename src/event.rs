//! Event handling.

use data::ItemDatabase;
use seed::prelude::*;

use crate::model::Model;

/// Possible events.
pub enum Msg {
    /// Item database has finished downloading.
    DataLoaded(ItemDatabase),
}

/// Reacts to events.
pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataLoaded(db) => model.data = Some(db),
    }
}
