use anyhow::{Context, Result};
use clap::Parser;

use dev_rs::{Cli, Commands, LambdaCommands, LambdaNamespace};

pub(crate) mod cache;
pub(crate) mod commands;
pub(crate) mod dirs;
pub(crate) mod settings;

fn run() -> Result<()> {
    let args = Cli::parse();

    let cache_dir = dirs::user_cache_dir().context("Cannot find cache directory")?;
    fs_err::create_dir_all(&cache_dir)?;
    let cache = cache::Cache::from_path(cache_dir);
    let settings = settings::FilesystemSettings::user()?;
    println!("{settings:?}");

    match *args.command {
        Commands::Lambda(LambdaNamespace {
            command: None,
            global_args,
        }) => {
            println!("fzf");
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
            println!("{global_args:?}");
            commands::lambda_fetch(args.path.as_deref(), cache)
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
