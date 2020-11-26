//! Adjust item data and apply projection.

mod third_age;

use std::sync::Mutex;

use data::Item;
use lazy_static::lazy_static;

use crate::osrsbox::ItemProperties;

lazy_static! {
    /// Map of item names to Trailblazer region expressions.
    static ref TRAILBLAZER_MAP: Mutex<trailblazer::data::ExprMap> =
        Mutex::new(trailblazer::data::create_map());
}

/// Apply all transformation methods.
pub fn map(item: ItemProperties) -> Result<Item, String> {
    let mut item = item.project()?;

    third_age::apply_value(&mut item);

    item.trailblazer = TRAILBLAZER_MAP.lock().unwrap().remove(&item.name);

    Ok(item)
}

/// Print leftover names in the Trailblazer item map (probably typos).
pub fn check() {
    third_age::check();

    for (name, _) in TRAILBLAZER_MAP.lock().unwrap().iter() {
        println!("Missed Trailblazer map: {}", name);
    }
}
