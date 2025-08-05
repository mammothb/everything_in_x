use clap::{ArgAction, Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum When {
    Auto,
    Always,
    Only,
    Never,
}

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
    /// read input from the files specified by NUL-terminated names in file F;
    /// If F is - then read names from standard input
    #[arg(long = "files0-from", value_name = "F")]
    files_from: Option<String>,
    /// print the maximum display width
    #[arg(short = 'L', long, action)]
    max_line_length: bool,
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

#[derive(Debug)]
struct Config {
    file_paths: Vec<String>,
    print_bytes: bool,
    print_chars: bool,
    print_lines: bool,
    print_words: bool,
    print_linelength: bool,
    files_from: Option<String>,
    total_mode: When,
}

fn main() {
    let args = Cli::parse();
    println!("{args:#?}");
}
