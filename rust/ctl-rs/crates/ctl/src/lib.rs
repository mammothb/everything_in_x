pub(crate) mod commands;
pub(crate) mod settings;

use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use ctl_options::FilesystemOptions;
use tracing_subscriber::FmtSubscriber;

use crate::{
    commands::{DisplayUrlType, ExitStatus},
    settings::{GlobalSettings, LambdaFetchSettings},
};
use ctl_cache::Cache;
use ctl_cli::{
    AwsCommands, AwsNamespace, Cli, Commands, LambdaCommands, LambdaNamespace,
};
use ctl_dirs::user_cache_dir;

#[allow(clippy::print_stderr)]
pub async fn main() -> ExitCode {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Tracing setup error: {error}");
        return ExitStatus::Failure.into();
    }
    let result = run().await;
    match result {
        Ok(code) => code.into(),
        Err(err) => {
            eprintln!("Error: {err}");
            ExitStatus::Success.into()
        }
    }
}

async fn run() -> Result<ExitStatus> {
    let cli = Cli::parse();

    let cache_dir = user_cache_dir().context("Cannot find cache directory")?;
    let cache = Cache::from_path(cache_dir);
    let filesystem = FilesystemOptions::user()?;
    let globals =
        GlobalSettings::resolve(&cli.global_args, filesystem.as_ref())?;

    match *cli.command {
        Commands::Aws(AwsNamespace {
            command: AwsCommands::Down,
        }) => Ok(ExitStatus::Success),

        Commands::Aws(AwsNamespace {
            command: AwsCommands::Up(args),
        }) => Ok(ExitStatus::Success),

        Commands::Lambda(LambdaNamespace { command: None }) => {
            let lambda_name = commands::lambda_find(
                &globals.environment,
                &globals.suffix,
                &cache,
            )?;
            commands::lambda_display_url(
                &lambda_name,
                &DisplayUrlType::Function,
            )
        }

        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Deps),
        }) => Ok(ExitStatus::Success),

        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Fetch(args)),
        }) => {
            let settings = LambdaFetchSettings::resolve(
                args,
                filesystem.as_ref(),
                globals.environment.clone(),
                globals.suffix.clone(),
                globals.verbose,
            );
            commands::lambda_fetch(&settings, &cache).await?;
            let lambda_name = commands::lambda_find(
                &globals.environment,
                &globals.suffix,
                &cache,
            )?;
            commands::lambda_display_url(
                &lambda_name,
                &DisplayUrlType::Function,
            )
        }

        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Log),
        }) => {
            let lambda_name = commands::lambda_find(
                &globals.environment,
                &globals.suffix,
                &cache,
            )?;
            commands::lambda_display_url(&lambda_name, &DisplayUrlType::Log)
        }

        Commands::Run => Ok(ExitStatus::Success),
    }
}
