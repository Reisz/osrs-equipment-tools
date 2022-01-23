use data::{RequirementType, Skill};
use serde::Deserialize;

/// Allows plain variant for untagged enum.
fn deserialize_combat<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    const NAME: &str = "combat";
    struct V;
    impl<'de> serde::de::Visitor<'de> for V {
        type Value = ();
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str(&format!("\"{NAME}\""))
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            (value == NAME)
                .then(|| ())
                .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Str(value), &self))
        }
    }
    deserializer.deserialize_str(V)
}

/// [OSRSBox](https://www.osrsbox.com/) requirement type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum ItemRequirement {
    /// A skill requirement.
    Skill(Skill),
    /// A combat level requirement.
    #[serde(deserialize_with = "deserialize_combat")]
    CombatLevel,
}

impl From<ItemRequirement> for RequirementType {
    fn from(req: ItemRequirement) -> Self {
        match req {
            ItemRequirement::Skill(s) => Self::Skill(s),
            ItemRequirement::CombatLevel => Self::CombatLevel,
        }
    }
}
