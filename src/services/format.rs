use crate::constants::ansi_codes::*;

pub fn colorize(b: bool) {
    if b {
        println!("{}{}{}", BRIGHT_GREEN, b, RESET);
    } else {
        println!("{}{}{}", BRIGHT_RED, b, RESET);
    }
}
