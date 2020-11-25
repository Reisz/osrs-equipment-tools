//! Adjust item data and apply projection.

use std::sync::Mutex;

use data::Item;
use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

lazy_static! {
    static ref TRAILBLAZER_MAP: Mutex<trailblazer::data::ExprMap> =
        Mutex::new(trailblazer::data::create_map());
}

/// Apply all transformation methods.
pub fn map(item: ItemProperties) -> Result<Item, String> {
    item.project().map(|mut i| {
        i.trailblazer = TRAILBLAZER_MAP.lock().unwrap().remove(&i.name);
        i
    })
}
