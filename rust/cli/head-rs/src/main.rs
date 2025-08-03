use std::process;

use anyhow::Result;
use clap::{ArgAction, Parser};
use head_rs::{XToIntFlag, xnumtoint};

const AFTER_HELP: &str = "NUM may have a multiplier suffix: b 512, kB 1000,
K 1024, MB 1000*1000, M 1024*1024, GB 1000*1000*1000, G 1024*1024*1024, and
so on for T, P, E, Z, Y, R, Q.  Binary prefixes can be used, too:
KiB=K, MiB=M, and so on.";

/// head - output the first part of files
#[derive(Debug, Parser)]
#[command(
    name = "head",
    bin_name = "head",
    version = "0.0.1",
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = AFTER_HELP,
)]
struct Cli {
    /// print the first NUM bytes of each file; with the leading '-', print all
    /// but the last NUM bytes of each file
    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "[-]NUM",
        conflicts_with = "num_lines"
    )]
    num_bytes: Option<String>,
    /// print the first NUM lines instead of the first 10; with the leading
    /// '-', print all but the last NUM lines of each file
    #[arg(short = 'n', long = "lines", value_name = "[-]NUM")]
    num_lines: Option<String>,
    /// never print headers giving file names
    #[arg(short, long, visible_alias = "silent", action)]
    quiet: bool,
    /// always print headers giving file names
    #[arg(short, long, action)]
    verbose: bool,
    /// line delimiter is NUL, not new line
    #[arg(short, long, action)]
    zero_terminated: bool,
    /// display this help and exit
    #[arg(long, action = ArgAction::Help)]
    help: Option<bool>,
    /// output version information and exit
    #[arg(long, action = ArgAction::Version)]
    version: Option<bool>,
    /// With no FILE, or when FILE is -, read standard input.
    #[arg(name = "FILE", default_value = "-")]
    file_paths: Vec<String>,
}

#[derive(Debug)]
struct Config {
    file_paths: Vec<String>,
    count_lines: bool,
    elide_from_end: bool,
    n_units: usize,
}

impl Config {
    fn from_args(args: Cli) -> Result<Self> {
        let (count_lines, value) = if let Some(bytes) = args.num_bytes {
            (false, bytes)
        } else {
            (true, args.num_lines.unwrap_or(String::from("10")))
        };
        let (elide_from_end, n_units) = if let Some(stripped) = value.strip_prefix('-') {
            (true, string_to_integer(stripped))
        } else {
            (false, string_to_integer(&value))
        };

        Ok(Self {
            file_paths: args.file_paths,
            count_lines,
            elide_from_end,
            n_units,
        })
    }
}

fn unwrap_or_exit<T, E>(result: Result<T, E>) -> T
where
    E: std::fmt::Display,
{
    match result {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}

fn string_to_integer(n_string: &str) -> usize {
    xnumtoint(
        n_string,
        0,
        i64::MAX,
        Some("bEGKkMmPQRTYZ0"),
        XToIntFlag::MaxQuiet,
    )
    .unwrap_or(i64::MAX) as usize
}

fn run(config: Config) -> Result<()> {
    Ok(())
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
    let config = unwrap_or_exit(Config::from_args(args));
    println!("{config:?}");
    unwrap_or_exit(run(config));
}
