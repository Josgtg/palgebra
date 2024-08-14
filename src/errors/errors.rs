fn get_err_message(code: u32) -> String {
    match code {
        _ => String::from("Syntax error")
    }
}

pub fn report(message: &str, code: u32, col: u32) {
    println!("{}: In character {}: {}", get_err_message(code), col, message);
}