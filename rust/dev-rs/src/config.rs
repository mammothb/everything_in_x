use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::settings::{FilesystemSettings, LambdaSettings, Settings};
use dev_rs::{
    LambdaFetchArgs, LambdaGlobalArgs,
    types::{Environment, StackSuffix},
};

#[derive(Clone, Debug)]
pub(crate) struct LambdaConfig {
    pub(crate) environment: Environment,
    pub(crate) suffix: StackSuffix,
    pub(crate) verbose: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct LambdaFetchConfig {
    pub(crate) definition_path: Option<PathBuf>,
    pub(crate) config: LambdaConfig,
}

impl LambdaFetchConfig {
    pub(crate) fn resolve(
        args: LambdaFetchArgs,
        global_args: LambdaGlobalArgs,
        filesystem_settings: Option<FilesystemSettings>,
    ) -> Result<Self> {
        let Settings { lambda } = filesystem_settings
            .map(FilesystemSettings::into_settings)
            .unwrap_or_default();
        let LambdaSettings {
            environment,
            suffix,
        } = lambda.unwrap_or_default();

        let config = Self {
            definition_path: args.path,
            config: LambdaConfig {
                environment: global_args.environment.unwrap_or(environment),
                suffix: global_args.suffix.unwrap_or(suffix),
                verbose: global_args.verbose,
            },
        };
        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.environment != Environment::Dev
            && self.config.suffix != StackSuffix::NoSuffix
        {
            return Err(anyhow!(
                "'{}' environment cannot be used with '{}' suffix",
                self.config.environment,
                self.config.suffix
            ));
        }
        Ok(())
    }
}
