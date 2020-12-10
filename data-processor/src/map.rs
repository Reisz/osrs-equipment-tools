//! Adjust item data and apply projection.

mod clues;
mod third_age;

use std::sync::Mutex;

use data::Item;
use lazy_static::lazy_static;

use crate::aggregate;
use crate::osrsbox::ItemProperties;

#[cfg(feature = "trailblazer")]
lazy_static! {
    /// Map of item names to Trailblazer region expressions.
    static ref TRAILBLAZER_MAP: Mutex<regions::data::ExprMap> =
        Mutex::new(regions::data::create_map());
}

/// Apply all transformation methods.
pub fn map(mut item: ItemProperties) -> Result<Item, String> {
    #[cfg(feature = "trailblazer")]
    let trailblazer = TRAILBLAZER_MAP
        .lock()
        .unwrap()
        .remove(item.wiki_name.as_ref().unwrap());

    aggregate::apply_aggregation(&mut item);

    let mut item = item.project()?;

    clues::apply_value(&mut item);
    third_age::apply_value(&mut item);

    #[cfg(feature = "trailblazer")]
    {
        item.trailblazer = trailblazer;
    }

    Ok(item)
}

/// Print leftover names in the Trailblazer item map (probably typos).
pub fn check() {
    aggregate::check();
    clues::check();
    third_age::check();

    #[cfg(feature = "trailblazer")]
    for (name, _) in TRAILBLAZER_MAP.lock().unwrap().iter() {
        println!("Missed Trailblazer map: {}", name);
    }
}
