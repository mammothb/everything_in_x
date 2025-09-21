use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::types::{Environment, StackSuffix};

pub mod types;

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

    #[command(flatten)]
    pub global_args: LambdaGlobalArgs,
}

#[derive(Debug, Parser)]
pub struct LambdaGlobalArgs {
    #[arg(short, long, value_enum)]
    pub suffix: Option<StackSuffix>,
    #[arg(short, long, value_enum)]
    pub environment: Option<Environment>,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum LambdaCommands {
    /// List the Lambda's dependencies
    Deps,
    /// Fetch Lambda function names
    Fetch(LambdaFetchArgs),
    /// Open CloudWatch logs
    Log,
}

#[derive(Args, Debug)]
pub struct LambdaFetchArgs {
    /// Path to infra definition file
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
