pub fn report(message: &str, code: u32, line: u32) {
    println!("{} in character {}: {}", code, line, message);
}