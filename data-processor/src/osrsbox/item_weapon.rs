use data::{AttackStyle, CombatOption, DamageType, WeaponData};
use serde::Deserialize;

/// [OSRSBox](https://www.osrsbox.com/) [`ItemWeapon`](https://www.osrsbox.com/projects/osrsbox-db/#item-weapon).
#[derive(Debug, Clone, Deserialize)]
pub struct ItemWeapon {
    /// The attack speed of a weapon (in game ticks).
    pub attack_speed: u8,
    /// The weapon classification (e.g., axes)
    pub weapon_type: String,
    /// An array of weapon stance information.
    pub stances: Vec<Stance>,
}

impl From<ItemWeapon> for WeaponData {
    fn from(weapon: ItemWeapon) -> Self {
        Self {
            attack_delay: weapon.attack_speed,
            combat_options: weapon.stances.into_iter().map(Stance::into).collect(),
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) attack style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsrsboxAttackStyle {
    /// Accurate attack style.
    Accurate,
    /// Aggressive attack style.
    Aggressive,
    /// Controlled attack style.
    Controlled,
    /// Defensive attack style.
    Defensive,
    /// Magic attack style (auto-cast).
    Magic,
}

impl From<OsrsboxAttackStyle> for AttackStyle {
    fn from(style: OsrsboxAttackStyle) -> Self {
        match style {
            OsrsboxAttackStyle::Accurate => Self::Accurate,
            OsrsboxAttackStyle::Aggressive => Self::Aggressive,
            OsrsboxAttackStyle::Controlled => Self::Controlled,
            OsrsboxAttackStyle::Defensive => Self::Defensive,
            OsrsboxAttackStyle::Magic => {
                unreachable!("magic is handled differently due to defensive auto-casting")
            }
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) attack type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsrsboxAttackType {
    /// Stab damage.
    Stab,
    /// Slash damage.
    Slash,
    /// Crush damage.
    Crush,
    /// Ranged damage.
    Ranged,
    /// Magic damage.
    Magic,
    /// Auto-casting.
    Spellcasting,
    /// Defensive auto-casting.
    #[serde(rename = "defensive casting")]
    DefensiveCasting,
}

impl From<OsrsboxAttackType> for DamageType {
    fn from(attack_type: OsrsboxAttackType) -> Self {
        match attack_type {
            OsrsboxAttackType::Stab => Self::Stab,
            OsrsboxAttackType::Slash => Self::Slash,
            OsrsboxAttackType::Crush => Self::Crush,
            OsrsboxAttackType::Ranged => Self::Ranged,
            _ => unreachable!(),
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) `ItemWeaponStance` (see [Swagger UI](https://api.osrsbox.com/swaggerui)).
#[derive(Debug, Clone, Deserialize)]
pub struct Stance {
    /// The name of the stance displayed in the interface.
    pub combat_style: String,
    /// The type of damage dealt by the attack.
    pub attack_type: Option<OsrsboxAttackType>,
    /// The attack style as displayed on the tooltip in the interface.
    pub attack_style: Option<OsrsboxAttackStyle>,
    /// The types of experience gained by using this stance.
    pub experience: Option<String>,
    /// The invisible boosts from the stance.
    pub boosts: Option<String>,
}

impl From<Stance> for CombatOption {
    fn from(stance: Stance) -> Self {
        let (style, damage_type) = match (
            stance.attack_type,
            stance.attack_style,
            stance.combat_style.as_str(),
            stance.experience.as_deref(),
        ) {
            (Some(OsrsboxAttackType::Spellcasting), _, _, _) => {
                (AttackStyle::AutoCast, DamageType::Magic)
            }
            (Some(OsrsboxAttackType::DefensiveCasting), _, _, _) => {
                (AttackStyle::DefensiveAutoCast, DamageType::Magic)
            }
            (Some(attack_type), Some(attack_style), _, _) => {
                (attack_style.into(), attack_type.into())
            }
            (_, _, "accurate", Some("ranged")) => (AttackStyle::Accurate, DamageType::Ranged),
            (_, _, "rapid", Some("ranged")) => (AttackStyle::Rapid, DamageType::Ranged),
            (_, _, "longrange", Some("ranged")) => (AttackStyle::LongRange, DamageType::Ranged),
            (_, _, "accurate", Some("magic")) => (AttackStyle::Accurate, DamageType::Magic),
            (_, _, "longrange", Some("magic")) => (AttackStyle::LongRange, DamageType::Magic),
            _ => unreachable!("Unknown combat option."),
        };

        let name = stance.combat_style;
        let name = name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .chain(name.chars().skip(1))
            .collect();

        Self {
            name,
            style,
            damage_type,
        }
    }
}
