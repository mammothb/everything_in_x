use anyhow::{Context, Result};
use clap::Parser;

use dev_rs::{
    Cli, Commands, CtlCommand, CtlNamespace, LambdaCommands, LambdaNamespace,
};
use tracing_subscriber::FmtSubscriber;

pub(crate) mod cache;
pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod dirs;
pub(crate) mod settings;

async fn run() -> Result<()> {
    let args = Cli::parse();

    let cache_dir =
        dirs::user_cache_dir().context("Cannot find cache directory")?;
    fs_err::create_dir_all(&cache_dir)?;
    let cache = cache::Cache::from_path(cache_dir);
    let settings = settings::FilesystemSettings::user()?;
    println!("{settings:?}");

    match *args.command {
        Commands::Ctl(CtlNamespace {
            command: CtlCommand::Down2,
        }) => {}
        Commands::Ctl(CtlNamespace {
            command: CtlCommand::Up(ctl_up_args),
        }) => {}
        Commands::Ctl(CtlNamespace {
            command: CtlCommand::Up2,
        }) => {}
        Commands::Lambda(LambdaNamespace { command: None }) => {
            let cache = cache.init()?;
            let config =
                config::LambdaConfig::resolve(*args.global_args, settings)?;
            let lambda_name = commands::lambda_find(&config, &cache)?;
            commands::lambda_display_url(
                &lambda_name,
                commands::LambdaDisplayUrlType::Function,
            )?;
        }
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Deps),
        }) => {}
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Fetch(lambda_fetch_args)),
        }) => {
            let cache = cache.init()?;
            let config = config::LambdaFetchConfig::resolve(
                lambda_fetch_args,
                *args.global_args,
                settings,
            )?;
            commands::lambda_fetch(&config, &cache).await?;
            let lambda_name = commands::lambda_find(&config.config, &cache)?;
            commands::lambda_display_url(
                &lambda_name,
                commands::LambdaDisplayUrlType::Function,
            )?;
        }
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Log),
        }) => {
            let cache = cache.init()?;
            let config =
                config::LambdaConfig::resolve(*args.global_args, settings)?;
            let lambda_name = commands::lambda_find(&config, &cache)?;
            commands::lambda_display_url(
                &lambda_name,
                commands::LambdaDisplayUrlType::Log,
            )?;
        }
        Commands::Run => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Tracing setup error: {error}");
        std::process::exit(1);
    }

    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
