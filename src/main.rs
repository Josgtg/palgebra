// #![allow(warnings)]

mod tests;

mod scanner;
mod parser;
mod token;
mod errors;
mod grammar;

use std::io::{self, BufRead};

fn welcome() {
    let mut message: String = String::new();
    message += "Propositional algebra evaluator\n";
    message += "Write a proposition and it will be evaluated to know if it is a well formed formula.\n";
    message += "----------------------------\nSymbol list:\n";
    message += "and: &\nor: |\nnot: !\nif and only if: ~\nif, then: >\n----------------------------\n";
    message += "Any alphabetical letter will be interpreted as a statement\n";
    message += "You can group using parenthesis\n";
    message += "Write your expression in the next line:\n";
    println!("{}", message);
}

fn read_expression() -> String {
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    expression
}

fn main() {
    welcome();

    let proposition = read_expression();

    let (tokens, err) = scanner::scan(&proposition);

    let res = parser::parse(tokens);

    println!();

    if err {
        println!("The proposition has errors; is not a WFF");
    } else if let Err(_) = res {
        println!("The proposition has errors; is not a WFF");
    } else {
        println!("Good! The proposition is a WFF");
    }
}