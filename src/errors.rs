fn get_err_message(code: u32) -> String {
    match code {
        0 => String::from("SyntaxError"),
        _ => String::from("ParseError")
    }
}

pub fn report(message: &str, code: u32, col: u32) {
    println!("{}: In character {}: {}", get_err_message(code), col, message);
}