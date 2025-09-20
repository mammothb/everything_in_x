use anyhow::{Context, Result};
use clap::Parser;

use dev_rs::{Cli, Commands, LambdaCommands, LambdaNamespace};

pub(crate) mod cache;
pub(crate) mod commands;
pub(crate) mod dirs;

fn run() -> Result<()> {
    let args = Cli::parse();

    let cache_dir = dirs::user_cache_dir().context("Cannot find cache directory")?;
    fs_err::create_dir_all(&cache_dir)?;
    let cache = cache::Cache::from_path(cache_dir);

    match *args.command {
        Commands::Lambda(LambdaNamespace { command: None }) => {
            println!("fzf");
        }
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Deps),
        }) => {}
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Fetch(args)),
        }) => commands::lambda_fetch(args.path.as_deref()),
        Commands::Lambda(LambdaNamespace {
            command: Some(LambdaCommands::Log),
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
