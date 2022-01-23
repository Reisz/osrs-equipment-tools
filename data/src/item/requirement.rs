use serde::{Deserialize, Serialize};

/// Enumeration of all skills in Old-school RuneScape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
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

/// Requirement in skill or combat level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Type {
    /// A skill requirement.
    Skill(Skill),
    /// A combat level requirement.
    CombatLevel,
}

/// A level requirement on a single skill.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Requirement {
    /// The type of requirement.
    pub requirement: Type,
    /// Required level.
    pub level: u8,
}
