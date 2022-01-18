//! Item sorting settings.

use std::cmp::Ordering;

use data::{Item, Stats};
use enum_iterator::IntoEnumIterator;
use seed::prelude::{LocalStorage, Orders, WebStorage};
use serde::{Deserialize, Serialize};

use super::Msg as SuperMsg;

const STORAGE_KEY: &str = "sorting";

/// Fragments for building a sorting method.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Fragment {
    /// Average of melee attack bonuses
    MeleeAttackAvg,
    /// Magic attack bonus
    MagicAttack,
    /// Ranged attack bonus
    RangedAttack,
    /// Median of all defensive bonuses
    DefenceMedian,
    /// Sort by melee strength
    MeleeStrength,
    /// Sort by ranged strength
    RangedStrength,
    /// Sort by magic damage
    MagicDamage,
    /// Sort by prayer bonus
    Prayer,
}

impl Fragment {
    fn get(self, i: &Stats) -> i16 {
        match self {
            Self::MeleeAttackAvg => {
                let a = &i.attack;
                (a.stab + a.slash + a.crush) / 3
            }
            Self::MagicAttack => i.attack.magic,
            Self::RangedAttack => i.attack.ranged,
            Self::DefenceMedian => {
                let d = &i.defence;
                let mut stats = [d.stab, d.slash, d.crush, d.magic, d.ranged];
                stats.sort_unstable();
                stats[2]
            }
            Self::MeleeStrength => i.melee_strength,
            Self::RangedStrength => i.ranged_strength,
            Self::MagicDamage => i.magic_damage,
            Self::Prayer => i.prayer,
        }
    }

    fn ordering(self, a: &Stats, b: &Stats) -> Ordering {
        let ord = self.get(a).cmp(&self.get(b));
        ord.reverse()
    }
}

/// Presets for sorting
#[derive(Debug, Clone, Copy, IntoEnumIterator)]
pub enum Preset {
    /// Prioritize strength bonus -> attack avg -> prayer -> defence median
    Melee,
    /// Prioritize damage bonus -> attack -> prayer -> defence median
    Magic,
    /// Prioritize strength bonus -> attack -> prayer -> defence median
    Ranged,
    /// Prioritize prayer -> defence median
    Prayer,
}

const MELEE_PRESET: &[Fragment] = &[
    Fragment::MeleeStrength,
    Fragment::MeleeAttackAvg,
    Fragment::DefenceMedian,
    Fragment::Prayer,
];

const MAGIC_PRESET: &[Fragment] = &[
    Fragment::MagicDamage,
    Fragment::MagicAttack,
    Fragment::DefenceMedian,
    Fragment::Prayer,
];

const RANGED_PRESET: &[Fragment] = &[
    Fragment::RangedStrength,
    Fragment::RangedAttack,
    Fragment::DefenceMedian,
    Fragment::Prayer,
];

const PRAYER_PRESET: &[Fragment] = &[Fragment::Prayer, Fragment::DefenceMedian];

impl Preset {
    /// Apply this preset to `sorting`.
    pub fn apply_to(self, sorting: &mut Sorting) {
        sorting.0.clear();
        let list = match self {
            Self::Melee => MELEE_PRESET,
            Self::Magic => MAGIC_PRESET,
            Self::Ranged => RANGED_PRESET,
            Self::Prayer => PRAYER_PRESET,
        };
        sorting.0.extend_from_slice(list);
    }
}

/// Stores current settings for item sorting.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sorting(Vec<Fragment>);

impl Sorting {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    #[must_use]
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    fn updated(&self) {
        LocalStorage::insert(STORAGE_KEY, self).unwrap();
    }

    /// Get an ordering between items `a` and `b` based on current settings.
    ///
    /// This method will always impose alphabetical ordering as a last step.
    #[must_use]
    pub fn ordering(&self, a: &Item, b: &Item) -> Ordering {
        let mut ordering = Ordering::Equal;

        for frag in &self.0 {
            ordering = ordering.then_with(|| frag.ordering(&a.stats, &b.stats));
        }

        let ordering = ordering.then_with(|| a.clue.cmp(&b.clue));
        ordering.then_with(|| a.name.cmp(&b.name))
    }

    /// Returns `true` if the item is better than an item with neutral stats under the
    /// current sorting order.
    #[must_use]
    pub fn above_neutral(&self, i: &Item) -> bool {
        let mut ordering = Ordering::Equal;

        let default = Stats::default();
        for frag in &self.0 {
            ordering = ordering.then_with(|| frag.ordering(&i.stats, &default));
        }

        ordering == Ordering::Less
    }
}

impl Default for Sorting {
    fn default() -> Self {
        let mut result = Self(Vec::new());
        Preset::Melee.apply_to(&mut result);
        result
    }
}

/// Messages to manipulate sorting order
pub enum Msg {
    /// Apply a preset to the sorting order.
    ApplyPreset(Preset),
}

/// Change sorting based on [`SortingMsg`].
pub fn update(msg: &Msg, sorting: &mut Sorting, _orders: &mut impl Orders<SuperMsg>) {
    match msg {
        Msg::ApplyPreset(preset) => preset.apply_to(sorting),
    }
    sorting.updated();
}
