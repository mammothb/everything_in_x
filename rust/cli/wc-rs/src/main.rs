use std::io::BufRead;

use anyhow::{Error, Result};
use clap::{ArgAction, Parser, ValueEnum};
use common::{open, unwrap_or_exit};

#[derive(Clone, Debug, PartialEq, ValueEnum)]
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
    #[arg(name = "FILE")]
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

impl Config {
    fn from_args(args: Cli) -> Result<Self> {
        let mut print_bytes = false;
        let mut print_lines = false;
        let mut print_words = false;

        if !(args.lines || args.words || args.chars || args.bytes || args.max_line_length) {
            print_bytes = true;
            print_lines = true;
            print_words = true;
        }
        if args.files_from.is_some() && !args.file_paths.is_empty() {
            return Err(Error::msg(
                "file operands cannot be combined with --files0-from",
            ));
        }
        Ok(Self {
            file_paths: args.file_paths,
            print_bytes,
            print_chars: args.chars,
            print_lines,
            print_words,
            print_linelength: args.max_line_length,
            files_from: args.files_from,
            total_mode: args.total,
        })
    }
}

struct ArgIter {
    // file-mode
    reader: Box<dyn BufRead>,
    token: Vec<u8>,
}

impl ArgIter {
    pub fn from_stream(reader: Box<dyn BufRead>) -> Self {
        Self {
            reader,
            token: Vec::new(),
        }
    }
}

impl Iterator for ArgIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.token.clear();
        self.reader
            .read_until(b'\0', &mut self.token)
            .ok()
            .filter(|&n| n > 0)
            .and_then(|_| String::from_utf8(self.token.clone()).ok())
    }
}

fn run(config: Config) -> Result<()> {
    let arg_iter: Box<dyn Iterator<Item = String>>;
    let mut files_from_stdin = false;
    if let Some(files_from) = config.files_from {
        files_from_stdin = files_from == "-";
        let reader = open(&files_from)?;
        arg_iter = Box::new(ArgIter::from_stream(reader));
    } else {
        arg_iter = Box::new(config.file_paths.into_iter());
    }
    let number_width = if config.total_mode == When::Only {
        1
    } else {
        2
    };
    let mut ok = true;
    for file_name in arg_iter {
        if files_from_stdin && file_name == "-" {
            eprintln!(
                "Error: when reading file names from standard input, no file names of {file_name} allowed"
            );
            ok = false;
            continue;
        }
        if file_name.is_empty() {
            eprintln!("Error: invalid zero-length file name");
            ok = false;
            continue;
        }
        ok = ok && wc(&file_name).is_ok();
    }
    if ok {
        Ok(())
    } else {
        Err(Error::msg("an unexpected error occurred"))
    }
}

fn wc(file_name: &str) -> Result<()> {
    println!("{file_name}");
    let reader = open(file_name)?;
    Ok(())
}

fn main() {
    let args = Cli::parse();
    println!("{args:#?}");
    let config = unwrap_or_exit(Config::from_args(args));
    println!("{config:#?}");
    unwrap_or_exit(run(config));
}
