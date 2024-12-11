pub mod codes;

use crate::constants::ansi_codes::*;
use std::fmt::Display;

pub enum Error {
    Syntax,
    Parse,
    File,
    VarAmount,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::Syntax => "SyntaxError",
            Error::Parse => "ParseError",
            Error::File => "FileError",
            Error::VarAmount => "VarAmountError",
        })
    }
}

pub fn scanner(message: &str, kind: Error, line: u32, col: u32) {
    eprintln!(
        "{}{}: line {}, column {}: {}{}",
        YELLOW, kind, line, col, message, RESET
    );
}

pub fn warn(message: &str, kind: Error) {
    eprintln!("{}{}: {}{}", BRIGHT_YELLOW, kind, message, RESET);
}

pub fn fatal(message: &str, kind: Error, exit_code: i32) {
    eprintln!("{}{}: {}{}", BRIGHT_YELLOW, kind, message, RESET);
    std::process::exit(exit_code);
}
