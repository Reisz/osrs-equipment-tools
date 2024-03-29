//! Datastructures for representing the applicaton state in [`Model`].

pub mod filter;
#[cfg(feature = "trailblazer")]
pub mod region_filter;
pub mod sorting;

use data::{Database, EquipSlot, Item};
use lzma_rs::xz_decompress;
use seed::prelude::*;
use web_sys::RequestCache;

use filter::Filter;
#[cfg(feature = "trailblazer")]
use region_filter::{RegionFilter, TrailblazerMsg};
use sorting::{Msg as SortingMsg, Sorting};

/// The application state.
#[derive(Default)]
pub struct Model {
    data: Option<Database>,
    sorting: Sorting,
    /// Slot currently shown by list view.
    pub list: Option<EquipSlot>,
    /// Miscellanious filtering
    pub filter: Filter,
    /// Item filtering based on trailblazer regions
    #[cfg(feature = "trailblazer")]
    pub trailblazer: RegionFilter,
}

impl Model {
    #[cfg(not(feature = "trailblazer"))]
    fn new() -> Self {
        Self {
            data: None,
            sorting: Sorting::new(),
            list: None,
            filter: Filter::new(),
        }
    }

    #[cfg(feature = "trailblazer")]
    fn new() -> Self {
        Self {
            data: None,
            sorting: Sorting::new(),
            list: None,
            filter: Filter::new(),
            trailblazer: RegionFilter::new(),
        }
    }

    /// Returns `true` when no data is present in the model.
    #[must_use]
    pub fn is_loading(&self) -> bool {
        self.data.is_none()
    }

    /// Get item at `index` in `slot`. Filters and sorting will be applied.
    ///
    /// Panics if [`is_loading()`] returns `true`.
    #[must_use]
    pub fn get_item(&self, slot: EquipSlot, idx: usize) -> Option<&Item> {
        self.iter(slot).nth(idx)
    }

    /// Get an iterator for the items in `slot`.
    ///
    /// # Panics
    ///
    /// When the data is not available.
    pub fn iter(&self, slot: EquipSlot) -> impl Iterator<Item = &Item> {
        self.data.as_ref().unwrap()[slot]
            .iter()
            .filter(move |i| self.filter(i))
            .take_while(move |i| self.sorting.above_neutral(i))
    }

    #[cfg(not(feature = "trailblazer"))]
    fn filter(&self, item: &Item) -> bool {
        self.filter.keep(item)
    }

    #[cfg(feature = "trailblazer")]
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
    orders.perform_cmd(async { Msg::DataLoaded(load_data().await.into()) });
    Model::new()
}

async fn load_data() -> Database {
    let request = Request::new("items.bin.xz").cache(RequestCache::NoCache);
    let response = request.fetch().await.unwrap();
    let bytes = response.bytes().await.unwrap();

    let mut decompressed = Vec::new();
    xz_decompress(&mut bytes.as_slice(), &mut decompressed).unwrap();
    bincode::deserialize(&decompressed).unwrap()
}

/// Possible events.
pub enum Msg {
    /// Item database has finished downloading.
    DataLoaded(Box<Database>),
    /// Change the current slot of the list view.
    ChangeList(EquipSlot),
    /// Message to change region-based filtering.
    #[cfg(feature = "trailblazer")]
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
            model.data = Some(*data);
            model.sort();
        }
        Msg::ChangeList(slot) => model.list = Some(slot),
        #[cfg(feature = "trailblazer")]
        Msg::Trailblazer(msg) => region_filter::update(msg, &mut model.trailblazer, orders),
        Msg::Sorting(msg) => {
            sorting::update(&msg, &mut model.sorting, orders);
            model.sort();
        }
    }
}
