//! Performs the following tasks:
//! 1. Download item data from the [OSRSBox](https://www.osrsbox.com/) static JSON API
//! 2. Filter the items
//!     - TODO
//! 3. Aggregate similar items
//!     - TODO
//! 4. Project to a more concise set of data-points
//! 5. Store in the compact bincode format
//! 6. Compress using the XZ algorithm
//!
//! The final product is stored in `data/items.bin.xz`.

#![deny(missing_docs)]

pub mod filter;
pub mod map;
pub mod osrsbox;

use std::{collections::HashMap, fs::File};

use data::ItemDatabase;
use xz2::write::XzEncoder;

use osrsbox::ItemProperties;

#[doc(hidden)]
fn main() {
    println!("Downloading...");
    let response =
        reqwest::blocking::get("https://www.osrsbox.com/osrsbox-db/items-complete.json").unwrap();
    let data: HashMap<String, ItemProperties> = response.json().unwrap();

    println!("Converting...");
    let items: ItemDatabase = data
        .into_iter()
        .filter_map(|i| {
            let id = i.1.id;
            if filter::keep(&i.1) {
                map::map(i.1).map_err(|e| println!("{}: {}", id, e)).ok()
            } else {
                None
            }
        })
        .collect();

    println!("Saving...");
    let output_file = File::create("data/items.bin.xz").unwrap();
    bincode::serialize_into(XzEncoder::new(output_file, 9), &items).unwrap();
}
