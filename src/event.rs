//! Event handling.

use data::ItemDatabase;
use seed::prelude::*;
use trailblazer::vars::Region;

use crate::model::Model;

/// Possible events.
pub enum Msg {
    /// Item database has finished downloading.
    DataLoaded(ItemDatabase),
    /// Toggle the trailblazer filter.
    ToggleTrailblazer,
    /// Toggle filtering of a region.
    ToggleRegion(Region),
}

/// Reacts to events.
pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataLoaded(db) => model.data = Some(db),
        Msg::ToggleTrailblazer => model.trailblazer.set_enabled(!model.trailblazer.enabled()),
        Msg::ToggleRegion(r) => model.trailblazer.toggle_region(r),
    }
}
