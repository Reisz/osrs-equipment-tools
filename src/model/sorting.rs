//! Item sorting settings.

use std::cmp::Ordering;

use data::Item;
use enum_iterator::IntoEnumIterator;
use seed::prelude::{LocalStorage, Orders, WebStorage};
use serde::{Deserialize, Serialize};

use super::Msg;

const STORAGE_KEY: &str = "sorting";

/// Fragments for building a sorting method.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SortingFragment {
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

impl SortingFragment {
    fn get(&self, i: &Item) -> i16 {
        match self {
            Self::MeleeAttackAvg => {
                let a = &i.stats.attack;
                (a.stab + a.slash + a.crush) / 3
            }
            Self::MagicAttack => i.stats.attack.magic,
            Self::RangedAttack => i.stats.attack.ranged,
            Self::DefenceMedian => {
                let d = &i.stats.defence;
                let mut stats = [d.stab, d.slash, d.crush, d.magic, d.ranged];
                stats.sort_unstable();
                stats[2]
            }
            Self::MeleeStrength => i.stats.melee_strength,
            Self::RangedStrength => i.stats.ranged_strength,
            Self::MagicDamage => i.stats.magic_damage.into(),
            Self::Prayer => i.stats.prayer,
        }
    }

    fn is_descending(&self) -> bool {
        true
    }

    fn ordering(&self, a: &Item, b: &Item) -> Ordering {
        let ord = self.get(a).cmp(&self.get(b));

        if self.is_descending() {
            ord.reverse()
        } else {
            ord
        }
    }
}

/// Presets for sorting
#[derive(Debug, Clone, Copy, IntoEnumIterator)]
pub enum SortingPreset {
    /// Prioritize strength bonus -> attack avg -> prayer -> defence median
    Melee,
    /// Prioritize damage bonus -> attack -> prayer -> defence median
    Magic,
    /// Prioritize strength bonus -> attack -> prayer -> defence median
    Ranged,
    /// Prioritize prayer -> defence median
    Prayer,
}

const MELEE_PRESET: &[SortingFragment] = &[
    SortingFragment::MeleeStrength,
    SortingFragment::MeleeAttackAvg,
    SortingFragment::Prayer,
    SortingFragment::DefenceMedian,
];

const MAGIC_PRESET: &[SortingFragment] = &[
    SortingFragment::MagicDamage,
    SortingFragment::MagicAttack,
    SortingFragment::Prayer,
    SortingFragment::DefenceMedian,
];

const RANGED_PRESET: &[SortingFragment] = &[
    SortingFragment::RangedStrength,
    SortingFragment::RangedAttack,
    SortingFragment::Prayer,
    SortingFragment::DefenceMedian,
];

const PRAYER_PRESET: &[SortingFragment] =
    &[SortingFragment::Prayer, SortingFragment::DefenceMedian];

impl SortingPreset {
    /// Apply this preset to `sorting`.
    pub fn apply_to(&self, sorting: &mut Sorting) {
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
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sorting(Vec<SortingFragment>);

impl Sorting {
    /// Create a new instance loaded from web storage or created with default values as fallback.
    pub fn new() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    fn updated(&self) {
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
pub enum SortingMsg {
    /// Apply a preset to the sorting order.
    ApplyPreset(SortingPreset),
}

/// Change sorting based on [`SortingMsg`].
pub fn update(msg: SortingMsg, sorting: &mut Sorting, _orders: &mut impl Orders<Msg>) {
    match msg {
        SortingMsg::ApplyPreset(preset) => preset.apply_to(sorting),
    }
    sorting.updated();
}
