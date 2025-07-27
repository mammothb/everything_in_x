use std::{iter::Peekable, str::Chars};

use clap::{ArgAction, Parser};
use itertools::Itertools;

/// echo - display a line of text
#[derive(Parser)]
#[command(
    name = "echo",
    bin_name = "echo",
    version = "0.0.1",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    /// do not output the trailing newline
    #[arg(short = 'n', action)]
    no_newline: bool,
    /// enable interpretation of backslash escapes
    /// the following sequences are recognized:
    ///   \\     backslash
    ///   \a     alert (BEL)
    ///   \b     backspace
    ///   \c     produce no further output
    ///   \e     escape
    ///   \f     form feed
    ///   \n     new line
    ///   \r     carriage return
    ///   \t     horizontal tab
    ///   \v     vertical tab
    ///   \0NNN  byte with octal value NNN (1 to 3 digits)
    ///   \xHH   byte with hexadecimal value HH (1 to 2 digits)
    #[arg(short = 'e', action, verbatim_doc_comment)]
    interpret_backslash: bool,
    /// disable interpretation of backslash escapes (default)
    #[arg(short = 'E', action)]
    no_interpret_backslash: bool,
    /// display this help and exit
    #[arg(long, action = ArgAction::Help)]
    help: Option<bool>,
    /// output version information and exit
    #[arg(long, action = ArgAction::Version)]
    version: Option<bool>,

    #[arg(name = "STRING", required = true)]
    words: Vec<String>,
}

enum EscapeAction {
    Emit(char),
    EmitStr(String),
    Stop,
    Skip,
}

fn parse_escape(chars: &mut Peekable<Chars>) -> Option<EscapeAction> {
    match chars.next()? {
        'a' => Some(EscapeAction::Emit('\x07')),
        'b' => Some(EscapeAction::Emit('\x08')),
        'c' => Some(EscapeAction::Stop),
        'e' => Some(EscapeAction::Emit('\x1B')),
        'f' => Some(EscapeAction::Emit('\x0C')),
        'n' => Some(EscapeAction::Emit('\n')),
        'r' => Some(EscapeAction::Emit('\r')),
        't' => Some(EscapeAction::Emit('\t')),
        'v' => Some(EscapeAction::Emit('\x0B')),
        'x' => {
            let hex = chars
                .peeking_take_while(|c| c.is_ascii_hexdigit())
                .take(2)
                .collect::<String>();
            match u8::from_str_radix(&hex, 16) {
                Ok(val) => Some(EscapeAction::Emit(val as char)),
                Err(_) => Some(EscapeAction::EmitStr(format!("\\x{hex}"))),
            }
        }
        '0' => {
            let oct = chars
                .peeking_take_while(|c| c.is_digit(8))
                .take(3)
                .collect::<String>();
            u8::from_str_radix(&oct, 8)
                .ok()
                .map(|val| EscapeAction::Emit(val as char))
        }
        '\\' => Some(EscapeAction::Emit('\\')),
        _ => Some(EscapeAction::Skip),
    }
}

fn main() {
    let args = Cli::parse();
    let interpret_backslash = match (args.interpret_backslash, args.no_interpret_backslash) {
        (true, false) => true,
        (false, true) | (false, false) => false,
        (true, true) => {
            eprintln!("Conflicting options: provided both '-e' and '-E'.");
            std::process::exit(1);
        }
    };
    let input = args.words.join(" ");
    let output = if interpret_backslash {
        let mut chars = input.chars().peekable();
        let mut result = String::new();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match parse_escape(&mut chars) {
                    Some(EscapeAction::Emit(c2)) => result.push(c2),
                    Some(EscapeAction::EmitStr(s)) => result.push_str(&s),
                    Some(EscapeAction::Stop) => return,
                    _ => continue,
                }
            } else {
                result.push(c);
            }
        }
        result
    } else {
        input
    };

    let ending = if args.no_newline { "" } else { "\n" };
    print!("{output}{ending}");
}
