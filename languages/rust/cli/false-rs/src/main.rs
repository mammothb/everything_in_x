use clap::{ArgAction, Parser};

/// false - do nothing, unsuccessfully
#[derive(Parser)]
#[command(
    name = "false",
    bin_name = "false",
    version = "0.0.1",
    allow_external_subcommands = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    /// display this help and exit
    #[arg(long, action = ArgAction::Help)]
    help: Option<bool>,
    /// output version information and exit
    #[arg(long, action = ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    Cli::parse();
    std::process::exit(1);
}
