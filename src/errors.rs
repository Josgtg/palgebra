fn get_err_message(code: u32) -> String {
    match code {
        0 => String::from("SyntaxError"),
        _ => String::from("ParseError")
    }
}

pub fn report(message: &str, code: u32, line: u32, col: u32) {
    println!("{}: line {}, character {}: {}", get_err_message(code), line, col, message);
}