//! Variable identifier and storage for Trailblazer League Regions. Used with [`BoolExpr`](crate::bool_expr::BoolExpr).

use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

/// Trailblazer regions. Misthalin and Karamja excluded as they are universal.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, IntoEnumIterator)]
pub enum Region {
    /// The Kingdom of Asgarnia
    Asgarnia,
    /// The Kharidian Desert
    Desert,
    /// The Fremennik Province
    Fremennik,
    /// The Kingdom of Kandarin
    Kandarin,
    /// The Land of Morytania
    Morytania,
    /// The Region of Tirannwn
    Tirannwn,
    /// The Wasteland of the Wilderness
    Wilderness,
}

impl TryFrom<&str> for Region {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, String> {
        match name {
            "A" => Ok(Self::Asgarnia),
            "D" => Ok(Self::Desert),
            "F" => Ok(Self::Fremennik),
            "K" => Ok(Self::Kandarin),
            "M" => Ok(Self::Morytania),
            "T" => Ok(Self::Tirannwn),
            "W" => Ok(Self::Wilderness),
            n => Err(format!("Unexpected name {}.", n)),
        }
    }
}

/// Stores any combiantion of the available regions.
///
/// ```
/// use std::convert::TryFrom;
/// # use regions::vars::{Region, RegionCombination};
/// use regions::bool_expr::BoolExpr;
///
/// let mut vars = RegionCombination::default();
///
/// let expr = BoolExpr::<Region>::try_from("A K &").unwrap();
/// assert_eq!(expr.eval(&vars), false);
///
/// vars[Region::Asgarnia] = true;
/// assert_eq!(expr.eval(&vars), false);
///
/// vars[Region::Kandarin] = true;
/// assert_eq!(expr.eval(&vars), true);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegionCombination([bool; Region::VARIANT_COUNT]);

impl RegionCombination {
    /// Returns true if `self` is a superset of `other`.
    #[must_use]
    pub fn is_superset_of(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(&other.0) {
            if !a && *b {
                return false;
            }
        }

        true
    }
}

impl From<u8> for RegionCombination {
    fn from(value: u8) -> Self {
        Self([
            value & 1 != 0,
            value & 2 != 0,
            value & 4 != 0,
            value & 8 != 0,
            value & 16 != 0,
            value & 32 != 0,
            value & 64 != 0,
        ])
    }
}

impl Index<&Region> for RegionCombination {
    type Output = bool;

    fn index(&self, region: &Region) -> &bool {
        &self.0[*region as usize]
    }
}

impl Index<Region> for RegionCombination {
    type Output = bool;

    fn index(&self, region: Region) -> &bool {
        &self.0[region as usize]
    }
}

impl IndexMut<Region> for RegionCombination {
    fn index_mut(&mut self, region: Region) -> &mut bool {
        &mut self.0[region as usize]
    }
}
