use anyhow::{Context, Result};
use clap::Parser;

use dev_rs::{Cli, Commands, LambdaCommands, LambdaNamespace};
use tracing_subscriber::FmtSubscriber;

pub(crate) mod cache;
pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod dirs;
pub(crate) mod settings;

fn run() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Cli::parse();

    let cache_dir = dirs::user_cache_dir().context("Cannot find cache directory")?;
    fs_err::create_dir_all(&cache_dir)?;
    let cache = cache::Cache::from_path(cache_dir);
    let settings = settings::FilesystemSettings::user()?;

    match *args.command {
        Commands::Lambda(LambdaNamespace {
            command: None,
            global_args,
        }) => {
            let cache = cache.init()?;
            let config = config::LambdaConfig::resolve(global_args, settings)?;
            commands::lambda_find(&config, &cache)?;
        }
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Deps),
            global_args,
        }) => {}
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Fetch(args)),
            global_args,
        }) => {
            let cache = cache.init()?;
            let config = config::LambdaFetchConfig::resolve(args, global_args, settings)?;
            commands::lambda_fetch(&config, &cache)?;
            commands::lambda_find(&config.config, &cache)?;
        }
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Log),
            global_args,
        }) => {}
        Commands::Run => {}
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
