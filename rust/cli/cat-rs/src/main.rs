use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use clap::{ArgAction, Parser};

/// cat - concatenate files and print on the standard output
#[derive(Parser)]
#[command(
    name = "cat",
    bin_name = "cat",
    version = "0.0.1",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    /// equivalent to -vET
    #[arg(short = 'A', long = "show-all", action)]
    show_all: bool,
    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", action)]
    number_nonblank: bool,
    /// equivalent to -vE
    #[arg(short = 'e', action)]
    show_ends_and_nonprinting: bool,
    /// display $ at end of each line
    #[arg(short = 'E', long = "show-ends", action)]
    show_ends: bool,
    /// number all output lines
    #[arg(short, long, action)]
    number: bool,
    /// suppress repeated empty output lines
    #[arg(short, long, action)]
    squeeze_blank: bool,
    /// equivalent to -vT
    #[arg(short = 't', action)]
    show_tabs_and_nonprinting: bool,
    /// display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", action)]
    show_tabs: bool,
    /// (ignored)
    #[arg(short = 'u', action)]
    _unbuffered: bool,
    /// use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long = "show-nonprinting", action)]
    show_nonprinting: bool,
    /// display this help and exit
    #[arg(long, action = ArgAction::Help)]
    help: Option<bool>,
    /// output version information and exit
    #[arg(long, action = ArgAction::Version)]
    version: Option<bool>,
    /// With no FILE, or when FILE is -, read standard input.
    #[arg(name = "FILE")]
    file_paths: Option<Vec<String>>,
}

#[derive(Debug)]
struct Config {
    /// files to contatenate
    file_paths: Vec<String>,
    /// number all output lines
    number: bool,
    /// number nonempty output lines
    number_nonblank: bool,
    /// display $ at end of each line
    show_ends: bool,
    /// use ^ and M- notation, except for LFD and TAB
    show_nonprinting: bool,
    /// display TAB characters as ^I
    show_tabs: bool,
    /// suppress repeated empty output lines
    squeeze_blank: bool,
}

fn run(config: Config) -> Result<()> {
    for file_path in config.file_paths {
        if file_path == "-" {
            continue;
        } else {
            let f = File::open(file_path)?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                if let Ok(content) = line {
                    println!("{content}");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    let config = Config {
        file_paths: args.file_paths.unwrap_or(vec![String::from("-")]),
        number: args.number,
        number_nonblank: args.number_nonblank,
        show_ends: args.show_ends || args.show_ends_and_nonprinting || args.show_all,
        show_nonprinting: args.show_nonprinting
            || args.show_ends_and_nonprinting
            || args.show_tabs_and_nonprinting
            || args.show_all,
        show_tabs: args.show_tabs || args.show_tabs_and_nonprinting || args.show_all,
        squeeze_blank: args.squeeze_blank,
    };
    if let Err(err) = run(config) {
        eprintln!("Error: {err}");
    }
}
