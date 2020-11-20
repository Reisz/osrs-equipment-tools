use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};

use enum_iterator::IntoEnumIterator;

use super::{Item, Slot};

/// Stores equipment seperated by eqip slots.
///
/// Use [`Index<Slot>`](#impl-Index<Slot>) and [`IndexMut<Slot>`](#impl-IndexMut<Slot>) implementations to
/// access individual slots.
#[derive(Debug, Clone, Default)]
pub struct EquipmentStorage([Vec<Item>; Slot::VARIANT_COUNT]);

impl EquipmentStorage {
    /// Sort storage for each slot individually.
    pub fn sort<F: FnMut(&Item, &Item) -> Ordering + Copy>(&mut self, compare: F) {
        for slot in &mut self.0 {
            slot.sort_unstable_by(compare);
        }
    }
}

impl Index<Slot> for EquipmentStorage {
    type Output = Vec<Item>;

    fn index(&self, slot: Slot) -> &Self::Output {
        &self.0[slot as usize]
    }
}

impl IndexMut<Slot> for EquipmentStorage {
    fn index_mut(&mut self, slot: Slot) -> &mut Self::Output {
        &mut self.0[slot as usize]
    }
}
