// #![allow(warnings)]

mod tests;

mod interpreter;
mod scanner;
mod parser;
mod token;
use token::Token;
mod errors;
mod grammar;

use std::{collections::{HashMap, HashSet}, io::{self, BufRead, Write}};

fn welcome() {
    let mut message: String = String::new();
    message += "\nPropositional algebra evaluator\n\n";
    message += "Write a proposition and it will be evaluated.\n";
    message += "----------------------------\nSymbol list:\n";
    message += "and: &\nor: |\nnot: !\nif and only if: ~\nif, then: >\n----------------------------\n";
    message += "Any alphabetical letter will be interpreted as a simple proposition.\n";
    message += "You can group using parenthesis.\n";
    message += "Write your expression in the next line:\n";
    println!("{}", message);
}

fn read_expression() -> String {
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    String::from(expression.trim())
}

fn read_bool(c: char) -> bool {
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

fn read_simples(simples: Vec<char>) -> HashMap<Token, bool> {
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

fn main() {
    welcome();

    let proposition = read_expression();
    if proposition.is_empty() {
        println!("Proposition is empty.");
        return
    }

    let (tokens, simples, err) = scanner::scan(&proposition);

    let res = parser::parse(tokens);

    println!();
    if err {
        println!("The proposition is not a well formed formula so is not possible to evaluate.");
        return
    } else if let Err(_) = res {
        println!("The proposition is not a well formed formula so is not possible to evaluate.");
        return
    } else {
        println!("Good! The proposition is a well formed formula.");
    }
    println!();

    let values = read_simples(simples);
    println!();

    let eval = interpreter::interpret(values, res.unwrap());
    println!("Your proposition evaluates to: {}", eval);
}