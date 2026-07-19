use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::{io, path::PathBuf};

use serde::Deserialize;

use ctl_aws_types::{AwsJob, Environment, StackSuffix};
use ctl_dirs::user_config_dir;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Options {
    #[serde(flatten)]
    pub global: Option<GlobalOptions>,
    pub lambda: Option<LambdaOptions>,
    pub aws: Option<AwsOptions>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalOptions {
    pub environment: Environment,
    pub suffix: StackSuffix,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LambdaOptions {
    pub stack_names: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AwsOptions {
    #[serde(flatten)]
    pub jobs: HashMap<String, AwsJob>,
}

#[derive(Clone, Debug)]
pub struct FilesystemOptions(Options);

impl Deref for FilesystemOptions {
    type Target = Options;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FilesystemOptions {
    #[must_use]
    pub fn into_options(self) -> Options {
        self.0
    }

    /// Loads the user ``FilesystemOptions``.
    ///
    /// # Errors
    ///
    /// Parsing the config TOML fails.
    pub fn user() -> Result<Option<Self>, Error> {
        let Some(config_dir) = user_config_dir() else {
            return Ok(None);
        };
        let config_path = config_dir.join("config.toml");
        match read_file(&config_path) {
            Ok(settings) => Ok(Some(Self(settings))),
            Err(Error::Io(err))
                if matches!(
                    err.kind(),
                    io::ErrorKind::NotFound
                        | io::ErrorKind::NotADirectory
                        | io::ErrorKind::PermissionDenied
                ) =>
            {
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Failed to parse: `{}`", _0.display())]
    Toml(PathBuf, #[source] Box<toml::de::Error>),
}

fn read_file(path: &Path) -> Result<Options, Error> {
    let content = fs_err::read_to_string(path)?;
    let settings = toml::from_str::<Options>(&content)
        .map_err(|err| Error::Toml(path.to_path_buf(), Box::new(err)))?;
    Ok(settings)
}
