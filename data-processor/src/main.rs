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

use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    time::Instant,
};

use data::ItemDatabase;
use xz2::write::XzEncoder;

use osrsbox::ItemProperties;

const DATA_URL: &str = "https://www.osrsbox.com/osrsbox-db/items-complete.json";

/// Print command name, time command execution and print timing.
pub fn measure<T>(name: &str, f: impl FnOnce() -> T) -> T {
    print!("{:30}", format!("{}...", name));
    io::stdout().flush().unwrap();

    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!(
        "{:5}.{:.2} ms",
        elapsed.as_millis(),
        format!("{:03}", elapsed.subsec_micros() % 1000)
    );

    result
}

/// Get the data from `items-complete.json`. Will look for `data/items-complete.json`, before
/// downloading from [OSRSBox](https://www.osrsbox.com/).
pub fn get_data() -> HashMap<String, ItemProperties> {
    if let Ok(mut input_file) = File::open("data/items-complete.json") {
        measure("Parsing file", || {
            // Using serde_json::from_reader is slower than this
            // (see https://github.com/serde-rs/json/issues/160)
            let mut s = String::new();
            input_file.read_to_string(&mut s).unwrap();
            serde_json::from_str(&s).unwrap()
        })
    } else {
        measure("Downloading & parsing", || {
            let response = reqwest::blocking::get(DATA_URL).unwrap();
            response.json().unwrap()
        })
    }
}

#[doc(hidden)]
fn main() {
    let data = get_data();

    println!("{:10} Items", data.len());

    let items: ItemDatabase = measure("Filtering & converting", || {
        data.into_iter()
            .filter_map(|i| {
                let id = i.1.id;
                if filter::keep(&i.1) {
                    map::map(i.1).map_err(|e| println!("{}: {}", id, e)).ok()
                } else {
                    None
                }
            })
            .collect()
    });

    println!("{:10} Items", items.len());

    measure("Saving", || {
        let output_file = File::create("data/items.bin.xz").unwrap();
        bincode::serialize_into(XzEncoder::new(output_file, 9), &items).unwrap();
    });
}
