//! Datastructures for representing the applicaton state in [`Model`].

pub mod region_filter;
pub mod sorting;

use data::ItemDatabase;
use lzma_rs::xz_decompress;
use seed::prelude::*;

use crate::event::Msg;
use region_filter::RegionFilter;
use sorting::{Sorting, SortingFragment};

/// The application state.
#[derive(Default)]
pub struct Model {
    data: Option<ItemDatabase>,
    sorting: Sorting,
    /// Item filtering based on trailblazer regions
    pub trailblazer: RegionFilter,
}

impl Model {
    /// Returns `true` when no data is present in the model.
    pub fn is_loading(&self) -> bool {
        self.data.is_none()
    }

    /// Get a reference to the database.
    ///
    /// Panics if [`is_loading()`] returns `true`.
    pub fn data(&self) -> &ItemDatabase {
        self.data.as_ref().unwrap()
    }

    /// Insert the data after it has been loaded.
    ///
    /// Panics if there is already data present.
    pub(crate) fn set_data(&mut self, data: ItemDatabase) {
        debug_assert!(self.data.is_none());
        self.data = Some(data);
    }

    /// Mutate the sorting order using a closure.
    ///
    /// The new ordering will automatically be applied to the database.
    pub fn map_sorting<F: FnOnce(&mut Vec<SortingFragment>)>(&mut self, f: F) {
        self.sorting.map(f);

        let mut data = self.data.take().unwrap();
        data.sort(|a, b| self.sorting.ordering(a, b));
        self.data = Some(data);
    }
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

    Model {
        data: None,
        trailblazer: RegionFilter::new(),
        sorting: Sorting::new(),
    }
}
