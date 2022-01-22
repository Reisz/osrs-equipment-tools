#[cfg(feature = "trailblazer")]
use regions::{bool_expr::BoolExpr, vars::Region};
use serde::{Deserialize, Serialize};

/// Denotes the available clue tiers.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize)]
pub enum Clue {
    /// Beginner clues.
    Beginner,
    /// Easy clues.
    Easy,
    /// Medium clues.
    Medium,
    /// Hard clues.
    Hard {
        /// Wether the item is in the mega-rare part of the drop-table.
        mega_rare: bool,
    },
    /// Elite clues.
    Elite {
        /// Wether the item is in the mega-rare part of the drop-table.
        mega_rare: bool,
    },
    /// Master clues.
    Master {
        /// Wether the item is in the mega-rare part of the drop-table.
        mega_rare: bool,
    },
}

/// Data about the attainability of an item. Useful for filtering.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attainability {
    /// Whether the item is tradeable. If true the other restrictions are not relevant for non-iron man accounts.
    pub tradeable: bool,
    /// The lowest clue tier required to obtain this item.
    pub clue: Option<Clue>,
    /// Trailblazer requirements.
    #[cfg(feature = "trailblazer")]
    pub trailblazer: Option<BoolExpr<Region>>,
}

impl Attainability {
    /// Create a new instance.
    #[must_use]
    pub fn new(tradeable: bool) -> Self {
        Self {
            tradeable,
            clue: None,
        }
    }
}
