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

    #[command(flatten)]
    pub global_args: Box<GlobalArgs>,
}

#[derive(Debug, Parser)]
pub struct GlobalArgs {
    /// Stack suffix
    #[arg(short, long, value_enum)]
    pub suffix: Option<StackSuffix>,
    /// Deployment environment
    #[arg(short, long, value_enum)]
    pub environment: Option<Environment>,
    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// IaC deploy/destroy
    Ctl(CtlNamespace),
    /// Lambda utilities
    Lambda(LambdaNamespace),
    /// Run commands through a custom environment
    Run,
}

#[derive(Args, Debug)]
pub struct CtlNamespace {
    #[command(subcommand)]
    pub command: CtlCommand,
}

#[derive(Debug, Subcommand)]
pub enum CtlCommand {
    Down2,
    Up(CtlUpArgs),
    Up2,
}

#[derive(Args, Debug)]
pub struct CtlUpArgs {
    /// Stacks to deploy
    #[arg(short, long)]
    pub stacks: Vec<String>,
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
