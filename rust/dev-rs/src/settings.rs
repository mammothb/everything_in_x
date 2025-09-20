use std::io;
use std::path::Path;

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::dirs::user_config_dir;
use dev_rs::types::{Environment, StackSuffix};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Settings {
    pub lambda: LambdaSettings,
}

impl Settings {
    pub(crate) fn validate(&self) -> Result<()> {
        self.lambda.validate()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct LambdaSettings {
    pub(crate) environment: Environment,
    pub(crate) suffix: StackSuffix,
}

impl LambdaSettings {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.environment != Environment::Dev && self.suffix != StackSuffix::NoSuffix {
            return Err(anyhow!(
                "'{}' environment cannot be used with '{}' suffix",
                self.environment,
                self.suffix
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FilesystemSettings(Settings);

impl FilesystemSettings {
    pub fn user() -> Result<Option<Self>> {
        let Some(config_dir) = user_config_dir() else {
            return Ok(None);
        };
        let config_path = config_dir.join("config.toml");
        match read_file(&config_path) {
            Ok(settings) => {
                settings.validate()?;
                Ok(Some(Self(settings)))
            }
            Err(err)
                if matches!(
                    err.downcast_ref::<io::Error>().map(|e| e.kind()),
                    Some(
                        io::ErrorKind::NotFound
                            | io::ErrorKind::NotADirectory
                            | io::ErrorKind::PermissionDenied
                    )
                ) =>
            {
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }
}

fn read_file(path: &Path) -> Result<Settings> {
    let content = fs_err::read_to_string(path)?;
    let settings = toml::from_str::<Settings>(&content)?;
    Ok(settings)
}
