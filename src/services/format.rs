#![allow(warnings)]

use crate::constants::ansi_codes::*;
use crate::types::TokenSequence;

pub fn colored_bool(b: bool) -> String {
    let color: &str;
    if b { color = BRIGHT_GREEN }
    else { color = BRIGHT_RED }
    format!("{}{}{}", color, b, RESET)
}

pub fn print_colored_bool(b: bool) {
    let color: &str;
    if b { color = BRIGHT_GREEN }
    else { color = BRIGHT_RED }
    println!("{}{}{}", color, b, RESET);
}

pub fn format_tokens(tokens: &TokenSequence) -> String {
    tokens.iter().map(|t| t.as_char()).collect()
}
