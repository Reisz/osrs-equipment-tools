use serde::{Deserialize, Serialize};

/// Enumeration of all skills in Old-school RuneScape.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    /// The Attack skill.
    Attack,
    /// The Hitpoints skill.
    Hitpoints,
    /// The Mining skill.
    Mining,
    /// The Strength skill.
    Strength,
    /// The Agility skill.
    Agility,
    /// The Smithing skill.
    Smithing,
    /// The Defence skill.
    Defence,
    /// The Herblore skill.
    Herblore,
    /// The Fishing skill.
    Fishing,
    /// The Ranged skill.
    Ranged,
    /// The Thieving skill.
    Thieving,
    /// The Cooking skill.
    Cooking,
    /// The Prayer skill.
    Prayer,
    /// The Crafting skill.
    Crafting,
    /// The Firemaking skill.
    Firemaking,
    /// The Magic skill.
    Magic,
    /// The Fletching skill.
    Fletching,
    /// The Woodcutting skill.
    Woodcutting,
    /// The Runecraft skill.
    Runecraft,
    /// The Slayer skill.
    Slayer,
    /// The Farming skill.
    Farming,
    /// The Construction skill.
    Construction,
    /// The Hunter skill.
    Hunter,
}

/// A level requirement on a single skill.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Requirement {
    /// Skill the level is required in.
    pub skill: Skill,
    /// Required level in the skill.
    pub level: u8,
}
