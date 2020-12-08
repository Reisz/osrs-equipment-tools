//! Item sorting settings.

use std::cmp::Ordering;

use data::Item;
use seed::prelude::{LocalStorage, Orders, WebStorage};
use serde::{Deserialize, Serialize};

use super::Msg;

const STORAGE_KEY: &str = "sorting";

/// Fragments for building a sorting method.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SortingFragment {
    /// Sort by melee strength
    MeleeStrength,
    /// Sort by ranged strength
    RangedStrength,
    /// Sort by magic damage
    MagicDamage,
    /// Sort by prayer bonus
    Prayer,
}

impl SortingFragment {
    fn ordering(&self, a: &Item, b: &Item) -> Ordering {
        match self {
            Self::MeleeStrength => a
                .stats
                .melee_strength
                .cmp(&b.stats.melee_strength)
                .reverse(),
            Self::RangedStrength => a
                .stats
                .ranged_strength
                .cmp(&b.stats.ranged_strength)
                .reverse(),
            Self::MagicDamage => a.stats.magic_damage.cmp(&b.stats.magic_damage).reverse(),
            Self::Prayer => a.stats.prayer.cmp(&b.stats.prayer).reverse(),
        }
    }
}

/// Stores current settings for item sorting.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sorting(Vec<SortingFragment>);

impl Sorting {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Mutate the sorting order using a closure.
    ///
    /// Automatically stores new state after mutation.
    pub fn map<F: FnOnce(&mut Vec<SortingFragment>)>(&mut self, f: F) {
        f(&mut self.0);
        LocalStorage::insert(STORAGE_KEY, self).unwrap();
    }

    /// Get an ordering between items `a` and `b` based on current settings.
    ///
    /// This method will always impose alphabetical ordering as a last step.
    pub fn ordering(&self, a: &Item, b: &Item) -> Ordering {
        let mut ordering = Ordering::Equal;

        for frag in &self.0 {
            ordering = ordering.then_with(|| frag.ordering(a, b));
        }

        ordering.then_with(|| a.name.cmp(&b.name))
    }
}

/// Messages to manipulate sorting order
pub enum SortingMsg {}

/// Change sorting based on [`SortingMsg`].
pub fn update(msg: SortingMsg, _sorting: &mut Sorting, _orders: &mut impl Orders<Msg>) {
    match msg {}
}
