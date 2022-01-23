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
            combat_options: weapon
                .stances
                .into_iter()
                .filter_map(Stance::into)
                .collect(),
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) attack style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemAttackStyle {
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

impl From<ItemAttackStyle> for AttackStyle {
    fn from(style: ItemAttackStyle) -> Self {
        match style {
            ItemAttackStyle::Accurate => Self::Accurate,
            ItemAttackStyle::Aggressive => Self::Aggressive,
            ItemAttackStyle::Controlled => Self::Controlled,
            ItemAttackStyle::Defensive => Self::Defensive,
            ItemAttackStyle::Magic => {
                unreachable!("magic is handled differently due to defensive auto-casting")
            }
        }
    }
}

/// [OSRSBox](https://www.osrsbox.com/) attack type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemAttackType {
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

impl From<ItemAttackType> for DamageType {
    fn from(attack_type: ItemAttackType) -> Self {
        match attack_type {
            ItemAttackType::Stab => Self::Stab,
            ItemAttackType::Slash => Self::Slash,
            ItemAttackType::Crush => Self::Crush,
            ItemAttackType::Ranged => Self::Ranged,
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
    pub attack_type: Option<ItemAttackType>,
    /// The attack style as displayed on the tooltip in the interface.
    pub attack_style: Option<ItemAttackStyle>,
    /// The types of experience gained by using this stance.
    pub experience: Option<String>,
    /// The invisible boosts from the stance.
    pub boosts: Option<String>,
}

impl From<Stance> for Option<CombatOption> {
    fn from(stance: Stance) -> Self {
        #[allow(clippy::match_same_arms)] // easier to organize
        let (style, damage_type) = match (
            stance.attack_type,
            stance.attack_style,
            stance.combat_style.as_str(),
            stance.experience.as_deref(),
        ) {
            // Autocasting
            (Some(ItemAttackType::Spellcasting), _, _, _) => {
                (AttackStyle::AutoCast, DamageType::Magic)
            }
            (Some(ItemAttackType::DefensiveCasting), _, _, _) => {
                (AttackStyle::DefensiveAutoCast, DamageType::Magic)
            }
            // Melee
            (Some(attack_type), Some(attack_style), _, _) => {
                (attack_style.into(), attack_type.into())
            }
            // [Salamander](https://oldschool.runescape.wiki/w/Salamander)- cases
            (Some(ItemAttackType::Slash), _, "scorch", Some("strength")) => (AttackStyle::Aggressive, DamageType::Slash),
            (Some(ItemAttackType::Ranged), _, "flare", Some("ranged")) => (AttackStyle::Accurate, DamageType::Ranged),
            (Some(ItemAttackType::Magic), _, "blaze", Some("magic")) => (AttackStyle::Defensive, DamageType::Magic),
            // [Chinchompa](https://oldschool.runescape.wiki/w/Chinchompa_(weapon))
            (_, _, "short fuse", Some("ranged")) => (AttackStyle::Accurate, DamageType::Ranged),
            (_, _, "medium fuse", Some("ranged")) => (AttackStyle::Rapid, DamageType::Ranged),
            (_, _, "long fuse", Some("ranged and defence")) => (AttackStyle::LongRange, DamageType::Ranged),
            // [Dinh's bulwark](https://oldschool.runescape.wiki/w/Dinh%27s_bulwark)
            (None, None, "block", None) => return None,
            // Ranged weapons
            (_, _, "accurate", Some("ranged")) => (AttackStyle::Accurate, DamageType::Ranged),
            (_, _, "rapid", Some("ranged")) => (AttackStyle::Rapid, DamageType::Ranged),
            (_, _, "longrange", Some("ranged and defence")) => {
                (AttackStyle::LongRange, DamageType::Ranged)
            }
            // Powered staves
            (_, _, "accurate", Some("magic")) => (AttackStyle::Accurate, DamageType::Magic),
            (_, _, "longrange", Some("magic and defence")) => {
                (AttackStyle::LongRange, DamageType::Magic)
            }
            _ => unreachable!(
                "Unknown combat option: type \"{:?}\", style \"{:?}\", name \"{}\", experience \"{:?}\".",
                stance.attack_type, stance.attack_style, stance.combat_style, stance.experience
            ),
        };

        let name = stance.combat_style;
        let name = name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .chain(name.chars().skip(1))
            .collect();

        Some(CombatOption {
            name,
            style,
            damage_type,
        })
    }
}
