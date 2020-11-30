//! Settings for region filtering.

use data::Item;
use seed::prelude::{LocalStorage, WebStorage};
use serde::{Deserialize, Serialize};
use trailblazer::vars::{Region, RegionCombination};

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

    /// Returns true if filtering is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the filtering.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated();
    }

    /// Returns a reference to the region settings.
    pub fn filter(&self) -> &RegionCombination {
        &self.filter
    }

    /// Toggle the filter state of `region`.
    pub fn toggle_region(&mut self, region: Region) {
        self.filter[region] = !self.filter[region];
        self.updated();
    }

    /// Returns false if the item is excluded by the current filter settings.
    pub fn evaluate(&self, item: &Item) -> bool {
        !self.enabled
            || item
                .trailblazer
                .as_ref()
                .map_or(true, |expr| expr.eval(&self.filter))
    }
}
