use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write, stdin},
};

use anyhow::Result;
use clap::{ArgAction, Parser};
use common::unwrap_or_exit;
use head_rs::{XToIntFlag, xnumtoint};

const READ_BUFSIZE: usize = 8192;
const BYTECOUNT_THRESHOLD: usize = 1024 * 1024;

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
    print_header: bool,
    line_end: char,
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
        let multiple_files = args.file_paths.len() > 1;

        Ok(Self {
            file_paths: args.file_paths,
            print_header: !args.quiet && (args.verbose || multiple_files),
            line_end: if args.zero_terminated { '\0' } else { '\n' },
            count_lines,
            elide_from_end,
            n_units,
        })
    }
}

struct State {
    first_file: bool,
}

impl State {
    fn new() -> Self {
        Self { first_file: true }
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

fn open(file_path: &str) -> Result<(Box<dyn BufRead>, &str)> {
    match file_path {
        "-" => Ok((Box::new(BufReader::new(stdin())), "standard input")),
        _ => Ok((Box::new(BufReader::new(File::open(file_path)?)), file_path)),
    }
}

fn run(config: Config) -> Result<()> {
    let mut state = State::new();
    for file_path in &config.file_paths {
        let result = open(file_path)
            .and_then(|(reader, filename)| head(reader, filename, &mut state, &config));
        if let Err(err) = result {
            eprintln!("cannot open {file_path} for reading: {err}");
        }
    }
    Ok(())
}

fn head(
    reader: Box<dyn BufRead>,
    filename: &str,
    state: &mut State,
    config: &Config,
) -> Result<()> {
    if config.print_header {
        println!(
            "{}==>{filename}<==",
            if state.first_file { "" } else { "\n" }
        );
        state.first_file = false;
    }

    if config.elide_from_end {
        if config.count_lines {
            elide_tail_lines(reader, config.line_end, config.n_units)?;
        } else {
            elide_tail_bytes(reader, config.n_units)?;
        }
    } else if config.count_lines {
        head_lines(reader, config.line_end, config.n_units)?;
    } else {
        head_bytes(reader, config.n_units)?;
    }

    Ok(())
}

fn head_bytes(reader: Box<dyn BufRead>, num_bytes: usize) -> Result<()> {
    let mut buf = Vec::new();
    reader.take(num_bytes as u64).read_to_end(&mut buf)?;
    print!("{}", String::from_utf8(buf)?);
    Ok(())
}

fn head_lines<T>(mut reader: Box<T>, line_end: char, mut num_lines: usize) -> Result<()>
where
    T: BufRead + ?Sized,
{
    let mut buf = Vec::new();
    while num_lines > 0 && reader.read_until(line_end as u8, &mut buf)? > 0 {
        print!("{}", String::from_utf8(buf.clone())?);
        buf.clear();
        num_lines -= 1;
    }
    Ok(())
}

fn elide_tail_bytes(mut reader: Box<dyn BufRead>, num_elide: usize) -> Result<()> {
    let mut stdout = io::stdout().lock();
    if num_elide <= BYTECOUNT_THRESHOLD {
        let buf_size = READ_BUFSIZE + num_elide;
        let mut buffers = vec![vec![0u8; buf_size]; 2];
        let mut i = 0;
        let mut first = true;

        let mut eof = false;
        while !eof {
            let n_read = reader.read(&mut buffers[i])?;
            let mut delta = 0;
            if n_read < buf_size {
                if n_read <= num_elide && !first {
                    delta = num_elide - n_read;
                }
                eof = true;
            }

            if !first {
                let start = READ_BUFSIZE;
                let to_write = num_elide - delta;
                stdout.write_all(&buffers[1 - i][start..start + to_write])?;
            }
            first = false;

            if n_read > num_elide {
                stdout.write_all(&buffers[i][..n_read - num_elide])?;
            }
            i = 1 - i;
        }
    } else {
        let remainer = num_elide % READ_BUFSIZE;
        let num_buffers = num_elide / READ_BUFSIZE + if remainer != 0 { 2 } else { 1 };
        let mut buffers: Vec<Vec<u8>> = Vec::with_capacity(num_buffers);
        let mut i = 0;
        let mut i_next = 1;
        let mut n_read = 0;

        let mut buffered_enough = false;
        let mut eof = false;
        while !eof {
            if !buffered_enough {
                buffers.push(vec![0u8; READ_BUFSIZE]);
            }
            n_read = reader.read(&mut buffers[i])?;
            if n_read < READ_BUFSIZE {
                eof = true;
            }
            if i + 1 == num_buffers {
                buffered_enough = true;
            }
            if buffered_enough {
                stdout.write_all(&buffers[i_next][..n_read])?;
            }
            i = i_next;
            i_next = (i_next + 1) % num_buffers;
        }

        let rem = READ_BUFSIZE - remainer;
        if buffered_enough {
            let rem_in_buffer = READ_BUFSIZE - n_read;
            if rem < rem_in_buffer {
                stdout.write_all(&buffers[i][n_read..n_read + rem])?;
            } else {
                stdout.write_all(&buffers[i][n_read..n_read + rem_in_buffer])?;
                stdout.write_all(&buffers[i_next][..rem - rem_in_buffer])?;
            }
        } else if i + 1 == num_buffers {
            // This happens when
            // n_elide < file_size < (n_bufs - 1) * READ_BUFSIZE.
            //
            // |READ_BUF.|
            // |                      |  rem |
            // |---------!---------!---------!---------|
            // |---- n_elide----------|
            // |                      | x |
            // |                   |y |
            // |---- file size -----------|
            // |                   |n_read|
            // |(n_bufs - 1) * READ_BUFSIZE--|

            let y = READ_BUFSIZE - rem;
            let x = n_read - y;
            stdout.write_all(&buffers[i_next][..x])?;
        }
    }
    Ok(())
}

fn elide_tail_lines(mut reader: Box<dyn BufRead>, line_end: char, num_elide: usize) -> Result<()> {
    let mut queue = VecDeque::with_capacity(num_elide);
    let mut buf = Vec::new();
    while reader.read_until(line_end as u8, &mut buf)? > 0 {
        if queue.len() == num_elide
            && let Some(string) = queue.pop_front()
        {
            print!("{string}");
        }
        queue.push_back(String::from_utf8(buf.clone())?);
        buf.clear();
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    let config = unwrap_or_exit(Config::from_args(args));
    unwrap_or_exit(run(config));
}
