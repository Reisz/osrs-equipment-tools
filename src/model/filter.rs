//! Miscellanious filters.

use data::Item;
use enum_iterator::IntoEnumIterator;
use seed::prelude::{LocalStorage, WebStorage};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "filter";

/// Available filtering options.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, IntoEnumIterator)]
pub enum FilterOption {
    /// Filters out third age equipment.
    ThirdAge,
    /// Filters out members equipment.
    Members,
}

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

    /// Returns true, if filtering by `option` is enabled.
    pub fn get(&self, option: FilterOption) -> bool {
        match option {
            FilterOption::ThirdAge => self.third_age,
            FilterOption::Members => self.members,
        }
    }

    /// Set filtering by `option` to be `enabled`.
    pub fn set(&mut self, option: FilterOption, enabled: bool) {
        *match option {
            FilterOption::ThirdAge => &mut self.third_age,
            FilterOption::Members => &mut self.members,
        } = enabled;
        LocalStorage::insert(STORAGE_KEY, self).unwrap();
    }

    /// Returns false if the item is excluded by the current filter settings.
    pub fn evaluate(&self, item: &Item) -> bool {
        !(self.third_age && item.third_age) && !(self.members && item.members)
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
