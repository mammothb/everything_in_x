use std::{collections::HashMap, fmt, path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Environment {
    #[default]
    Dev,
    Uat,
    Prd,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dev => write!(f, "dev"),
            Self::Uat => write!(f, "uat"),
            Self::Prd => write!(f, "prd"),
        }
    }
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

impl fmt::Display for StackSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dev1 => write!(f, "-dev1"),
            Self::Dev2 => write!(f, "-dev2"),
            Self::Dev3 => write!(f, "-dev3"),
            Self::Dev4 => write!(f, "-dev4"),
            Self::NoSuffix => write!(f, ""),
            Self::Custom(suffix) => write!(f, "-{suffix}"),
        }
    }
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
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AwsJob {
    pub steps: Vec<AwsStep>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AwsStep {
    pub cmd: String,
    pub chdir: Option<PathBuf>,
    pub executable: Option<PathBuf>,
    pub env: Option<HashMap<String, String>>,
}
