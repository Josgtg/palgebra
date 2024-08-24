use crate::errors;
use std::fs;
use std::io::{self, BufRead, Write};

pub fn colorize(b: bool) {
    if b {
            println!("\x1b[92m{}\x1b[0m", b);
    } else {
        println!("\x1b[91m{}\x1b[0m", b);
    }
}

pub fn read_expression_from_file(path: &str) -> String {
    let proposition = fs::read_to_string(path);
    if let Ok(s) = proposition {
        return s;
    }
    errors::fatal("file could not be read", 2, 1, true);
    String::new()
}

pub fn read_expression_from_user() -> String {
    print!(">>> ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("failed to read line, restart the program and try again");
    String::from(expression.trim())
}

pub fn divide_proposition(proposition: String) -> Vec<String> {
    // splits the tokens in vecs separated by new lines
    let mut vec = Vec::new();
    let mut curr: String = String::new();
    for c in proposition.chars() {
        if c == '\r' { continue; }
        if c == '\n' {
            if !curr.is_empty() {
                vec.push(curr);
                curr = String::new();
            }
            continue;
        }
        curr.push(c);
    }
    if !curr.is_empty() {
        vec.push(curr);
    }
    vec
}
