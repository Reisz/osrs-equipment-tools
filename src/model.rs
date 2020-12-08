//! Datastructures for representing the applicaton state in [`Model`].

pub mod filter;
pub mod region_filter;
pub mod sorting;

use data::{Item, ItemDatabase, Slot};
use lzma_rs::xz_decompress;
use seed::prelude::*;
use web_sys::RequestCache;

use filter::Filter;
use region_filter::{RegionFilter, TrailblazerMsg};
use sorting::{Sorting, SortingMsg};

/// The application state.
#[derive(Default)]
pub struct Model {
    data: Option<ItemDatabase>,
    sorting: Sorting,
    /// Miscellanious filtering
    pub filter: Filter,
    /// Item filtering based on trailblazer regions
    pub trailblazer: RegionFilter,
}

impl Model {
    /// Returns `true` when no data is present in the model.
    pub fn is_loading(&self) -> bool {
        self.data.is_none()
    }

    /// Get item at `index` in `slot`. Filters and sorting will be applied.
    ///
    /// Panics if [`is_loading()`] returns `true`.
    pub fn get_item(&self, slot: Slot, idx: usize) -> Option<&Item> {
        self.iter(slot).nth(idx)
    }

    /// Get the amount of items in `slot`.
    ///
    /// Can be slow, as the filters will be applied every time.
    pub fn slot_len(&self, slot: Slot) -> usize {
        self.iter(slot).count()
    }

    fn iter(&self, slot: Slot) -> impl Iterator<Item = &Item> {
        self.data.as_ref().unwrap()[slot]
            .iter()
            .filter(move |i| self.filter(i))
    }

    fn filter(&self, item: &Item) -> bool {
        self.filter.keep(item) && self.trailblazer.keep(item)
    }

    fn sort(&mut self) {
        let mut data = self.data.take().unwrap();
        data.sort(|a, b| self.sorting.ordering(a, b));
        self.data = Some(data);
    }
}

/// Initialize the model and start item data loading process.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let response = Request::new("data/items.bin.xz")
            .cache(RequestCache::NoCache)
            .fetch()
            .await
            .unwrap();
        let bytes = response.bytes().await.unwrap();

        let mut decompressed = Vec::new();
        xz_decompress(&mut bytes.as_slice(), &mut decompressed).unwrap();
        Msg::DataLoaded(bincode::deserialize(&decompressed).unwrap())
    });

    Model {
        data: None,
        sorting: Sorting::new(),
        filter: Filter::new(),
        trailblazer: RegionFilter::new(),
    }
}

/// Possible events.
pub enum Msg {
    /// Item database has finished downloading.
    DataLoaded(ItemDatabase),
    /// Message to change region-based filtering.
    Trailblazer(TrailblazerMsg),
    /// Message to change sorting behaviour.
    ///
    /// Will trigger a sort afterwards.
    Sorting(SortingMsg),
}

/// Reacts to events.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataLoaded(data) => {
            debug_assert!(model.data.is_none());
            model.data = Some(data);
            model.sort();
        }
        Msg::Trailblazer(msg) => region_filter::update(msg, &mut model.trailblazer, orders),
        Msg::Sorting(msg) => {
            sorting::update(msg, &mut model.sorting, orders);
            model.sort();
        }
    }
}
