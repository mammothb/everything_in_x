use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use ctl_aws_types::{Environment, StackSuffix};

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
    /// Deployment environment
    #[arg(short, long, value_enum)]
    pub environment: Option<Environment>,
    /// Stack suffix
    #[arg(short, long, value_enum)]
    pub suffix: Option<StackSuffix>,
    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Deploy/destroy resources
    Aws(AwsNamespace),
    /// Lambda utilities
    Lambda(LambdaNamespace),
    /// Run commands through a custom environment
    Run,
}

#[derive(Args, Debug)]
pub struct AwsNamespace {
    #[command(subcommand)]
    pub command: AwsCommands,
}

#[derive(Debug, Subcommand)]
pub enum AwsCommands {
    Down,
    Up(AwsUpArgs),
}

#[derive(Args, Debug)]
pub struct AwsUpArgs {
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
    /// Open Cloudwatch logs
    Log,
}

#[derive(Args, Debug)]
pub struct LambdaFetchArgs {
    /// Path to infra definition file
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
