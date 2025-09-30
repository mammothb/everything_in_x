use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use ctl_aws_types::{AwsStep, Environment, StackSuffix};
use ctl_cli::{AwsUpArgs, GlobalArgs, LambdaFetchArgs};
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

#[derive(Clone, Debug)]
pub(crate) struct AwsUpSettings {
    pub(crate) steps: Vec<AwsStep>,
    pub(crate) stack_names: Vec<String>,
    pub(crate) environment: Environment,
    pub(crate) suffix: StackSuffix,
    pub(crate) verbose: bool,
}

impl AwsUpSettings {
    pub(crate) fn resolve(
        args: AwsUpArgs,
        filesystem: Option<&FilesystemOptions>,
        environment: Environment,
        suffix: StackSuffix,
        verbose: bool,
    ) -> Result<Self> {
        let job_name = args.job.as_deref().unwrap_or("default");
        let steps = filesystem
            .and_then(|f| f.aws.clone())
            .map(|a| a.jobs)
            .context("No AWS jobs defined")?
            .get(job_name)
            .with_context(|| format!("Job `{job_name}` not defined"))?
            .steps
            .clone();
        let settings = Self {
            steps,
            stack_names: args.stack_names,
            environment,
            suffix,
            verbose,
        };
        Ok(settings)
    }
}

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
