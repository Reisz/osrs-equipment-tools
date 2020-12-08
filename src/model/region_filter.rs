//! Settings for region filtering.

use data::Item;
use regions::vars::{Region, RegionCombination};
use seed::prelude::{LocalStorage, Orders, WebStorage};
use serde::{Deserialize, Serialize};

use super::Msg;

const STORAGE_KEY: &str = "region-filter";

/// Stores current settings for region filtering.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RegionFilter {
    enabled: bool,
    filter: RegionCombination,
}

impl RegionFilter {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    fn updated(&self) {
        LocalStorage::insert(STORAGE_KEY, self).unwrap();
    }

    /// Returns `true` if filtering is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns a reference to the region settings.
    pub fn filter(&self) -> &RegionCombination {
        &self.filter
    }

    /// Returns `false` if the item is excluded by the current filter settings.
    pub fn keep(&self, item: &Item) -> bool {
        !self.enabled
            || item
                .trailblazer
                .as_ref()
                .map_or(true, |expr| expr.eval(&self.filter))
    }
}

/// Messages to maipulate region-based filters.
pub enum TrailblazerMsg {
    /// Enable / disable region filtering.
    ToggleEnabled,
    /// Enable / disable filtering of a region.
    ToggleRegion(Region),
}

/// Change region filters based on [`TrailblazerMsg`].
pub fn update(msg: TrailblazerMsg, filter: &mut RegionFilter, _orders: &mut impl Orders<Msg>) {
    match msg {
        TrailblazerMsg::ToggleEnabled => filter.enabled = !filter.enabled,
        TrailblazerMsg::ToggleRegion(r) => filter.filter[r] = !filter.filter[r],
    }

    filter.updated();
}
