//! Datastructures for representing the applicaton state in [`Model`].

use data::ItemDatabase;
use lzma_rs::xz_decompress;
use seed::prelude::*;

use crate::event::Msg;

/// The application state.
#[derive(Default)]
pub struct Model {
    /// Item database
    pub data: Option<ItemDatabase>,
}

/// Initialize the model and start item data loading process.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let response = fetch("data/items.bin.xz").await.unwrap();
        let bytes = response.bytes().await.unwrap();

        let mut decompressed = Vec::new();
        xz_decompress(&mut bytes.as_slice(), &mut decompressed).unwrap();
        Msg::DataLoaded(bincode::deserialize(&decompressed).unwrap())
    });

    Model::default()
}
