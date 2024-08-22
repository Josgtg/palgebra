use crate::token::Token;
use std::{collections::{HashMap, HashSet}, io::{self, BufRead, Write}};

pub fn read_expression() -> String {
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    String::from(expression.trim())
}

pub fn read_bool(c: char) -> bool {
    let mut input: String;
    let stdin = io::stdin();
    let mut boolean: Result<bool, _>;

    loop {
        print!("Enter value for {}: ", c);
        io::stdout().flush().expect("Failed to flush stdout. Restart the program and try again");

        input = String::new();
        stdin.lock().read_line(&mut input).expect("Failed to read line. Restart the program and try again");

        input = input.trim().parse().expect("");
        if input.eq("0") {
            return false
        }
        if input.eq("1") {
            return true
        }

        boolean = input.parse();
        if let Ok(b) = boolean {
            return b
        }

        println!("Value is invalid. Please enter either a 0 or a 1")
    }
}

pub fn read_simples(simples: Vec<char>) -> HashMap<Token, bool> {
    let mut seen: HashSet<char> = HashSet::new();
    let mut message = String::from("Now write the values of your simple propositions.\n");
    message += "Write a 0 for \"false\" and a 1 for \"true\". You can also just write \"false\" and \"true\":\n";
    println!("{}", message);

    let mut values: HashMap<Token, bool> = HashMap::new();

    for s in simples {
        if let Some(_) = seen.get(&s)  { continue }

        values.insert(Token::Sentence(s), read_bool(s));
        seen.insert(s);
    }

    values
}