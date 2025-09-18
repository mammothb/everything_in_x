use std::path::PathBuf;

use clap::{Parser, Subcommand};

use dev_rs::get_config_path;

#[derive(Debug, Parser)]
#[command(name = "dev")]
#[command(about = "Dev utility tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lambda utilities
    Lambda {
        #[command(subcommand)]
        command: Option<LambdaCommands>,
    },
    /// Run commands through a custom environment
    Run,
}

#[derive(Debug, Subcommand)]
enum LambdaCommands {
    /// List the Lambda's dependencies
    Deps,
    /// Fetch Lambda function names
    Fetch {
        /// Path to infra definition file
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Open CloudWatch logs
    Log,
}

fn main() {
    let args = Cli::parse();

    let config_path = get_config_path();
    println!("{config_path:?}");

    match args.command {
        Commands::Lambda { command } => {
            if let Some(command) = command {
                match command {
                    LambdaCommands::Deps => {}
                    LambdaCommands::Fetch { path } => {}
                    LambdaCommands::Log => {}
                }
            } else {
                println!("fzf");
            }
        }
        Commands::Run => {}
    }
}
