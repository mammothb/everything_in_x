use std::path::PathBuf;

use anyhow::{Result, bail};

use ctl_aws_types::{Environment, StackSuffix};
use ctl_cli::{GlobalArgs, LambdaFetchArgs};
use ctl_options::FilesystemOptions;

#[derive(Clone, Debug)]
pub(crate) struct GlobalSettings {
    pub(crate) environment: Environment,
    pub(crate) suffix: StackSuffix,
    pub(crate) verbose: bool,
}

impl GlobalSettings {
    pub(crate) fn resolve(
        args: &GlobalArgs,
        filesystem: Option<&FilesystemOptions>,
    ) -> Result<Self> {
        let settings = Self {
            environment: args
                .environment
                .clone()
                .or(filesystem
                    .and_then(|f| f.global.clone())
                    .map(|g| g.environment))
                .unwrap_or_default(),
            suffix: args
                .suffix
                .clone()
                .or(filesystem.and_then(|f| f.global.clone()).map(|g| g.suffix))
                .unwrap_or_default(),
            verbose: args.verbose,
        };
        settings.validate()?;
        Ok(settings)
    }

    fn validate(&self) -> Result<()> {
        if self.environment != Environment::Dev
            && self.suffix != StackSuffix::NoSuffix
        {
            bail!(
                "`{}` environment cannot be used with `{}` suffix",
                self.environment,
                self.suffix
            );
        }
        Ok(())
    }
}
// #[derive(Clone, Debug)]
// pub(crate) struct CtlSettings {
//     pub(crate) environment: Environment,
//     pub(crate) suffix: StackSuffix,
//     pub(crate) verbose: bool,
// }

#[derive(Clone, Debug)]
pub(crate) struct LambdaFetchSettings {
    pub(crate) path: Option<PathBuf>,
    pub(crate) stack_names: Vec<String>,
    pub(crate) environment: Environment,
    pub(crate) suffix: StackSuffix,
    pub(crate) verbose: bool,
}

impl LambdaFetchSettings {
    pub(crate) fn resolve(
        args: LambdaFetchArgs,
        filesystem: Option<&FilesystemOptions>,
        environment: Environment,
        suffix: StackSuffix,
        verbose: bool,
    ) -> Self {
        Self {
            path: args.path,
            stack_names: filesystem
                .and_then(|f| f.lambda.clone())
                .map(|l| l.stack_names)
                .unwrap_or_default(),
            environment,
            suffix,
            verbose,
        }
    }
}
