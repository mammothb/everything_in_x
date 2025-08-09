use anyhow::Result;
use clap::ValueEnum;
use std::{
    io::{self, BufRead, Write, stdout},
    ops::{Add, AddAssign},
};

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum When {
    Auto,
    Always,
    Only,
    Never,
}

impl When {
    pub fn should_print(&self, num_inputs: usize) -> bool {
        match self {
            Self::Auto => num_inputs > 1,
            Self::Always | Self::Only => true,
            Self::Never => false,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub file_paths: Vec<String>,
    pub print_bytes: bool,
    pub print_chars: bool,
    pub print_lines: bool,
    pub print_words: bool,
    pub total: When,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WordCount {
    num_bytes: usize,
    num_chars: usize,
    num_lines: usize,
    num_words: usize,
}

impl Add for WordCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num_bytes: self.num_bytes + rhs.num_bytes,
            num_chars: self.num_chars + rhs.num_chars,
            num_lines: self.num_lines + rhs.num_lines,
            num_words: self.num_words + rhs.num_words,
        }
    }
}

impl AddAssign for WordCount {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

pub fn print_stats(config: &Config, result: &WordCount, title: &str) -> io::Result<()> {
    let mut stdout = stdout().lock();
    let mut space = "";
    for (_, num) in [
        (config.print_lines, result.num_lines),
        (config.print_words, result.num_words),
        (config.print_chars, result.num_chars),
        (config.print_bytes, result.num_bytes),
    ]
    .iter()
    .filter(|(print, _)| *print)
    {
        write!(stdout, "{space}{num:>8}")?;
        space = " ";
    }
    writeln!(stdout, "{space}{title}")
}

pub fn wc<T>(mut reader: T) -> Result<WordCount>
where
    T: BufRead,
{
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        num_bytes += buf.len();
        num_chars += buf.chars().count();
        num_lines += 1;
        num_words += buf.split_whitespace().count();
        buf.clear();
    }
    Ok(WordCount {
        num_bytes,
        num_chars,
        num_lines,
        num_words,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_wc() {
        let text = "I don't want the world. I just want your half.\r\n";

        let result = wc(Cursor::new(text));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            WordCount {
                num_bytes: 48,
                num_chars: 48,
                num_lines: 1,
                num_words: 10
            }
        );
    }
}
