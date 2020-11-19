use serde::Deserialize;

/// Weapon-specific data.
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponData {
    #[serde(rename = "attack_speed")]
    attack_delay: u8,
    stances: Vec<WeaponStance>,
}

impl WeaponData {
    /// Delay between attacks in ticks.
    pub fn attack_delay(&self) -> u8 {
        self.attack_delay
    }

    /// Slice of available stances.
    pub fn stances(&self) -> &[WeaponStance] {
        &self.stances
    }
}

/// Weapon stance as selectable in the [combat options](https://oldschool.runescape.wiki/w/Combat_Options).
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponStance {
    #[serde(rename = "combat_style", deserialize_with = "deserialize_stance")]
    name: String,
    attack_type: AttackType,
    attack_style: AttackStyle,
}

impl WeaponStance {
    /// Stance name as displayed in the interface.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Damage type of the stance.
    pub fn attack_type(&self) -> AttackType {
        self.attack_type
    }

    /// Stance attack style as displayed in the popup.
    pub fn attack_style(&self) -> AttackStyle {
        self.attack_style
    }
}

/// [Types of attacks](https://oldschool.runescape.wiki/w/Attack_types) in RuneScape.
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AttackType {
    /// Stab damage
    Stab,
    /// Slash damage
    Slash,
    /// Crush damage
    Crush,
    /// Magic damage
    Magic,
    /// Ranged damage
    Ranged,
}

/// Attack styles as named on individual [combat options](https://oldschool.runescape.wiki/w/Combat_Options).
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AttackStyle {
    /// Accurate style
    Accurate,
    /// Aggressive style
    Aggressive,
    /// Controlled style
    Controlled,
    /// Defensive style
    Defensive,
    /// Magic style
    Magic,
}

fn deserialize_stance<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<String, D::Error> {
    struct UppercasingStringVistor;

    impl<'de> serde::de::Visitor<'de> for UppercasingStringVistor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a string")
        }

        fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
            let mut s = s.to_owned();
            s[0..1].make_ascii_uppercase();
            Ok(s)
        }
    }

    deserializer.deserialize_string(UppercasingStringVistor)
}
