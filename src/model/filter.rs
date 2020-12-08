//! Miscellanious filters.

use data::Item;
use seed::prelude::{LocalStorage, WebStorage};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "filter";

/// Stores current settings for general filtering.
///
/// Uses individual named fields to allow manual inspection / editing of local storage.
#[derive(Debug, Deserialize, Serialize)]
pub struct Filter {
    third_age: bool,
    members: bool,
}

impl Filter {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Returns `true` if 3rd age items are included in results.
    pub fn keeps_third_age(&self) -> bool {
        !self.third_age
    }

    /// Returns `true` if members items are included in results.
    pub fn keeps_members(&self) -> bool {
        !self.members
    }

    /// Returns `false` if the item is excluded by the current filter settings.
    pub fn keep(&self, item: &Item) -> bool {
        !(self.third_age && item.third_age || self.members && item.members)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            third_age: true,
            members: false,
        }
    }
}
