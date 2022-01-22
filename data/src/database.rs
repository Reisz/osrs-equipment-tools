use std::{
    cmp::Ordering,
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

use crate::{EquipSlot, Item};

/// Contains a database of all equipment items separated by equip slots.
///
/// Use [`Index<EquipSlot>`](#impl-Index<EquipSlot>) and [`IndexMut<EquipSlot>`](#impl-IndexMut<EquipSlot>)
/// implementations to access individual slots.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Database([Vec<Item>; EquipSlot::VARIANT_COUNT]);

impl Database {
    /// Sort each database slot individually.
    pub fn sort<F: FnMut(&Item, &Item) -> Ordering + Copy>(&mut self, compare: F) {
        for slot in &mut self.0 {
            slot.sort_unstable_by(compare);
        }
    }

    /// Get the total amount of items in the database.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.iter().fold(0, |acc, e| acc + e.len())
    }

    /// Return true if no item is stored in the database
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(Vec::is_empty)
    }
}

impl FromIterator<Item> for Database {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut result = Self::default();

        for item in iter {
            result[item.equip_slot].push(item);
        }

        result
    }
}

impl Index<EquipSlot> for Database {
    type Output = Vec<Item>;

    fn index(&self, slot: EquipSlot) -> &Self::Output {
        &self.0[slot as usize]
    }
}

impl IndexMut<EquipSlot> for Database {
    fn index_mut(&mut self, slot: EquipSlot) -> &mut Self::Output {
        &mut self.0[slot as usize]
    }
}
