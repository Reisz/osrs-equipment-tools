use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut},
};

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

/// All the damage types available to the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, IntoEnumIterator)]
pub enum DamageType {
    /// Melee stab damage.
    Stab,
    /// Melee slash damage.
    Slash,
    /// Melee crush damage.
    Crush,
    /// Magic damage.
    Magic,
    /// Ranged damage.
    Ranged,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Stab => "Stab",
            Self::Slash => "Slash",
            Self::Crush => "Crush",
            Self::Magic => "Magic",
            Self::Ranged => "Ranged",
        };
        write!(f, "{text}")
    }
}

/// Contains one value for each [`DamageType`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Stats([i16; DamageType::VARIANT_COUNT]);

impl Stats {
    /// Create a new instance from the values provided. Order is determined by [`DamageType`].
    #[must_use]
    pub fn new(values: &[i16; DamageType::VARIANT_COUNT]) -> Self {
        Self(*values)
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.0.iter_mut().zip(rhs.0) {
            *a += b;
        }
    }
}

impl Index<DamageType> for Stats {
    type Output = i16;

    fn index(&self, index: DamageType) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<DamageType> for Stats {
    fn index_mut(&mut self, index: DamageType) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stat_indices() {
        let stats = Stats::new(&[0, 1, 2, 3, 4]);
        for (i, damage_type) in DamageType::into_enum_iter().enumerate() {
            assert_eq!(stats[damage_type] as usize, i);
        }
    }

    #[test]
    fn stat_addition() {
        let a = Stats::new(&[1, 2, 3, 4, 5]);
        let b = Stats::new(&[6, 7, 8, 9, 10]);
        assert_eq!(a + b, Stats::new(&[7, 9, 11, 13, 15]));
    }
}
