use std::str::FromStr;

use serde::{Deserialize, Deserializer, de};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Environment {
    #[default]
    Dev,
    Uat,
    Prd,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "uat" => Ok(Self::Uat),
            "prd" => Ok(Self::Prd),
            _ => Err(format!("Unknown environment: {s}")),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum StackSuffix {
    #[default]
    Dev1,
    Dev2,
    Dev3,
    Dev4,
    NoSuffix,
    /// Custom suffix, typically '-<username>'
    Custom(String),
}

impl FromStr for StackSuffix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev1" => Ok(Self::Dev1),
            "dev2" => Ok(Self::Dev2),
            "dev3" => Ok(Self::Dev3),
            "dev4" => Ok(Self::Dev4),
            "-" => Ok(Self::NoSuffix),
            _ => {
                if s.chars().all(|c| c.is_ascii_lowercase()) {
                    Ok(Self::Custom(s.to_lowercase()))
                } else {
                    Err(format!("unknown stack suffix: {s}"))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for StackSuffix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StackSuffix::from_str(&s).map_err(de::Error::custom)
    }
}
