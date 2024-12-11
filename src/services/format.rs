#![allow(warnings)]

use crate::constants::ansi_codes::*;
use crate::types::TokenSequence;

pub fn colorize(b: bool) {
    if b {
        println!("{}{}{}", BRIGHT_GREEN, b, RESET);
    } else {
        println!("{}{}{}", BRIGHT_RED, b, RESET);
    }
}

pub fn format_tokens(tokens: TokenSequence) -> String {
    let mut s: String = String::new();
    for token in tokens.iter() {
        s.push(token.as_char());
    }
    s
}
