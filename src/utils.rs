use crate::token::Token;
use crate::errors;
use std::fs;
use std::io::{self, BufRead, Write};

pub fn colorize(i: u32, b: bool) {
    if i == 0 {
        if b {
            println!("\x1b[92m{}\x1b[0m", b);
        } else {
            println!("\x1b[91m{}\x1b[0m", b);
        }
    } else {
        if b {
            println!("{}: \x1b[92m{}\x1b[0m", i, b);
        } else {
            println!("{}: \x1b[91m{}\x1b[0m", i, b);
        }
    }
}

pub fn read_expression_from_file(path: &str) -> String {
    let proposition = fs::read_to_string(path);
    if let Ok(s) = proposition {
        return s;
    }
    errors::fatal("file could not be read", 3, 1);
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

pub fn divide_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    // splits the tokens in vecs separated by new lines
    let mut vec = Vec::new();
    let mut curr: Vec<Token> = Vec::new();
    for t in tokens {
        if t == Token::NewLine || t == Token::Comment {
            vec.push(curr);
            curr = Vec::new();
            continue;
        }
        curr.push(t);
    }
    if !curr.is_empty() {
        vec.push(curr);
    }
    vec
}
