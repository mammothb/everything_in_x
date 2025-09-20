use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dev")]
#[command(about = "Dev utility tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Box<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lambda utilities
    Lambda(LambdaNamespace),
    /// Run commands through a custom environment
    Run,
}

#[derive(Args, Debug)]
pub struct LambdaNamespace {
    #[command(subcommand)]
    pub command: Option<LambdaCommands>,
}

#[derive(Debug, Subcommand)]
pub enum LambdaCommands {
    /// List the Lambda's dependencies
    Deps,
    /// Fetch Lambda function names
    Fetch(FetchArgs),
    /// Open CloudWatch logs
    Log,
}

#[derive(Args, Debug)]
pub struct FetchArgs {
    /// Path to infra definition file
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
