use std::{
    cmp::Ordering,
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

use super::{Item, Slot};

/// Contains a database of all equipment items seperated by eqip slots.
///
/// Use [`Index<Slot>`](#impl-Index<Slot>) and [`IndexMut<Slot>`](#impl-IndexMut<Slot>) implementations to
/// access individual slots.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemDatabase([Vec<Item>; Slot::VARIANT_COUNT]);

impl ItemDatabase {
    /// Sort each database slot individually.
    pub fn sort<F: FnMut(&Item, &Item) -> Ordering + Copy>(&mut self, compare: F) {
        for slot in &mut self.0 {
            slot.sort_unstable_by(compare);
        }
    }

    /// Get the total amount of items in the database.
    pub fn len(&self) -> usize {
        self.0.iter().fold(0, |acc, e| acc + e.len())
    }

    /// Return true if no item is stored in the database
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|e| e.is_empty())
    }
}

impl FromIterator<Item> for ItemDatabase {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut result = Self::default();

        for item in iter {
            result[item.slot].push(item);
        }

        result
    }
}

impl Index<Slot> for ItemDatabase {
    type Output = Vec<Item>;

    fn index(&self, slot: Slot) -> &Self::Output {
        &self.0[slot as usize]
    }
}

impl IndexMut<Slot> for ItemDatabase {
    fn index_mut(&mut self, slot: Slot) -> &mut Self::Output {
        &mut self.0[slot as usize]
    }
}
