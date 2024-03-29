//! Miscellanious filters.

use data::Item;
use seed::prelude::{LocalStorage, WebStorage};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "filter";

/// Stores current settings for general filtering.
///
/// Uses individual named fields to allow manual inspection / editing of local storage.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Filter {
    members: bool,
}

impl Filter {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    #[must_use]
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Returns `false` if the item is excluded by the current filter settings.
    #[must_use]
    pub fn keep(&self, item: &Item) -> bool {
        !(self.members && item.members)
    }
}
