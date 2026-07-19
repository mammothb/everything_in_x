use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::process;

pub fn open(file_path: &str) -> Result<Box<dyn BufRead>> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

pub fn unwrap_or_exit<T, E>(result: Result<T, E>) -> T
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
