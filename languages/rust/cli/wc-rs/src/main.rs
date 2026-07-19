use anyhow::Result;
use clap::{ArgAction, Parser};
use common::{open, unwrap_or_exit};
use wc_rs::{Config, When, WordCount, print_stats, wc};

/// wc - print newline, word, and byte counts for each file
#[derive(Debug, Parser)]
#[command(
    name = "wc",
    bin_name = "wc",
    version = "0.0.1",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    /// print the byte counts
    #[arg(short = 'c', long, action)]
    bytes: bool,
    /// print the byte counts
    #[arg(short = 'm', long, action)]
    chars: bool,
    /// print the newline counts
    #[arg(short, long, action)]
    lines: bool,
    /// print the word counts
    #[arg(short, long, action)]
    words: bool,
    /// when to print a line with total counts
    #[arg(long, value_name = "WHEN", value_enum, default_value = "auto")]
    total: When,
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

fn get_args() -> Result<Config> {
    let args = Cli::parse();
    let print_chars = args.chars;
    let mut print_bytes = args.bytes;
    let mut print_lines = args.lines;
    let mut print_words = args.words;

    if [print_bytes, print_chars, print_lines, print_words]
        .iter()
        .all(|v| !v)
    {
        print_bytes = true;
        print_lines = true;
        print_words = true;
    }
    Ok(Config {
        file_paths: args.file_paths,
        print_bytes,
        print_chars,
        print_lines,
        print_words,
        total: args.total,
    })
}

fn run(config: Config) -> Result<()> {
    let mut total_word_count = WordCount::default();
    for file_path in &config.file_paths {
        match open(file_path).and_then(wc) {
            Ok(result) => {
                if let Err(err) = print_stats(&config, &result, file_path) {
                    eprintln!("{file_path}: {err}");
                    continue;
                }
                total_word_count += result;
            }
            Err(err) => eprintln!("{file_path}: {err}"),
        }
    }
    if config.total.should_print(config.file_paths.len()) {
        if let Err(err) = print_stats(&config, &total_word_count, "total") {
            eprintln!("total: {err}");
        }
    }
    Ok(())
}

fn main() {
    unwrap_or_exit(get_args().and_then(run));
}
