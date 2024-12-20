use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use crate::errors::{self, codes, Error};

pub fn read_expression_from_file(path: PathBuf) -> String {
    match fs::read_to_string(path) { 
        Ok(s) => return s,
        Err(e) => errors::fatal_detailed(
            "file could not be read",
            Error::File,
            codes::FILE_ERROR,
            e
        )
    };
    String::new()
}

pub fn read_expression_from_user() -> String {
    print!(">>> ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut expression)
        .expect("failed to read line, restart the program and try again");
    String::from(expression.trim())
}
