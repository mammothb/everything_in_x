use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader, stdin},
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

struct State {
    // number of newlines
    num_newlines: usize,
    /// the current line number
    line_num: usize,
}

fn cat<T>(mut reader: Box<T>, state: &mut State, config: &Config) -> Result<()>
where
    T: BufRead + ?Sized,
{
    let mut buf = String::new();
    let mut line_num = String::new();
    while reader.read_line(&mut buf)? > 0 {
        if buf == "\n" {
            state.num_newlines += 1;
        } else {
            state.num_newlines = 0;
        }
        if state.num_newlines > 0 {
            // Prevent potential overflow
            state.num_newlines = cmp::min(state.num_newlines, 2);
            if state.num_newlines >= 2 && config.squeeze_blank {
                buf.clear();
                line_num.clear();
                continue;
            }
            if config.number && !config.number_nonblank {
                state.line_num += 1;
                line_num.insert_str(0, &format_line_num(&state.line_num));
            }
        } else if config.number {
            state.line_num += 1;
            line_num.insert_str(0, &format_line_num(&state.line_num));
        }
        if config.show_ends || config.show_nonprinting || config.show_tabs {
            print!(
                "{line_num}{}",
                &buf.bytes()
                    .map(|c| format_char(c, config))
                    .collect::<String>()
            );
        } else {
            print!("{line_num}{buf}");
        }
        buf.clear();
        line_num.clear();
    }
    Ok(())
}

fn cat_simple<T>(mut reader: Box<T>) -> Result<()>
where
    T: BufRead + ?Sized,
{
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        print!("{buf}");
        buf.clear();
    }
    Ok(())
}

fn format_char(c: u8, config: &Config) -> String {
    if config.show_nonprinting {
        match c {
            b'\t' => {
                if config.show_tabs {
                    String::from("^I")
                } else {
                    String::from(c as char)
                }
            }
            b'\n' => {
                if config.show_ends {
                    String::from("$\n")
                } else {
                    String::from("\n")
                }
            }
            0..=31 => format!("^{}", (c + 64) as char),
            127 => String::from("^?"),
            128..=255 => format!("M-{}", format_char(c - 128, config)),
            _ => String::from(c as char),
        }
    } else if c == b'\t' && config.show_tabs {
        String::from("^I")
    } else if c == b'\n' && config.show_ends {
        String::from("$\n")
    } else {
        String::from(c as char)
    }
}

fn format_line_num(line_num: &usize) -> String {
    format!("{line_num:>6}\t")
}

fn open(file_path: &str) -> Result<Box<dyn BufRead>> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

fn run(config: Config) -> Result<()> {
    let mut state = State {
        num_newlines: 0,
        line_num: 0,
    };
    for file_path in &config.file_paths {
        let reader = open(file_path)?;
        if !(config.number
            || config.show_ends
            || config.show_nonprinting
            || config.show_tabs
            || config.squeeze_blank)
        {
            cat_simple(reader)?;
        } else {
            cat(reader, &mut state, &config)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    let config = Config {
        file_paths: args.file_paths.unwrap_or(vec![String::from("-")]),
        number: args.number || args.number_nonblank,
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
