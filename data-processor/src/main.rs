//! Performs the following tasks:
//! 1. Download item data from the [OSRSBox](https://www.osrsbox.com/) static JSON API
//! 2. Filter the items
//!     - Duplicates
//!     - Non-equippable items
//!     - Items with no positive stats
//!     - Others (see submodules of [filter])
//! 3. Aggregate similar items
//! 4. Append additional data
//! 5. Project to a more concise set of data-points
//! 6. Store in the compact bincode format
//! 7. Compress using the XZ algorithm
//!
//! The final product is stored in `data/items.bin.xz`.

#![deny(missing_docs)]

pub mod aggregate;
pub mod filter;
pub mod map;
pub mod osrsbox;

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Read, Write},
    time::Instant,
};

use data::ItemDatabase;
use xz2::write::XzEncoder;

use osrsbox::ItemProperties;

const DATA_URL: &str = "https://www.osrsbox.com/osrsbox-db/items-complete.json";
const CACHE_FILE: &str = "data/items-complete.json";
const OUTPUT_FILE: &str = "dist/items.bin.xz";

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
    if let Ok(mut input_file) = File::open(CACHE_FILE) {
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

/// Save a local copy of the item data to reduce downloads during development.
pub fn cache_data() {
    let response = reqwest::blocking::get(DATA_URL).unwrap();
    let mut file = File::create(CACHE_FILE).unwrap();
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();
}

/// Check whether the output file has to be regenerated based on file modification dates.
pub fn out_of_date() -> io::Result<bool> {
    let output_date = fs::metadata(OUTPUT_FILE)?.modified()?;
    let input_date = fs::metadata(CACHE_FILE)?.modified()?;

    if input_date > output_date {
        return Ok(true);
    }

    let exec = env::args()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, ""))?;
    let exec_date = fs::metadata(exec)?.modified()?;

    if exec_date > output_date {
        return Ok(true);
    }

    Ok(false)
}

#[doc(hidden)]
fn main() {
    if env::args().any(|arg| arg == "--cache") {
        measure("Downloading", cache_data);
        return;
    }

    if let Ok(false) = out_of_date() {
        println!("Nothing to do...");
        return;
    }

    let data = get_data();
    println!("{:10} Items", data.len());

    let mut errors = Vec::new();
    let items: ItemDatabase = measure("Filtering & converting", || {
        data.into_iter()
            .filter_map(|i| {
                let name = i.1.wiki_name.clone().unwrap_or_else(|| i.1.name.clone());
                if filter::keep(&i.1) {
                    map::map(i.1)
                        .map_err(|e| errors.push(format!("{}: {}", name, e)))
                        .ok()
                } else {
                    None
                }
            })
            .collect()
    });
    println!("{:10} Items", items.len());

    errors.iter().for_each(|e| println!("Error: {}", e));
    filter::check();
    map::check();

    measure("Saving", || {
        let output_file = File::create(OUTPUT_FILE).unwrap();
        bincode::serialize_into(XzEncoder::new(output_file, 9), &items).unwrap();
    });
}
