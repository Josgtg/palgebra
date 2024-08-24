fn get_err_message(code: u32) -> String {
    match code {
        0 => String::from("SyntaxError"),
        1 => String::from("ParseError"),
        2 => String::from("FileError"),
        _ => String::from("VarAmountError"),
    }
}

pub fn report(message: &str, code: u32, line: u32, col: u32) {
    eprintln!("\x1b[33m{}: line {}, character {}: {}\x1b[0m", get_err_message(code), line, col, message);
}

pub fn fatal(message: &str, code: u32, unix_code: i32, exit: bool) {
    eprintln!("\x1b[93m{}: {}\x1b[0m", get_err_message(code), message);
    if exit { std::process::exit(unix_code) }
}