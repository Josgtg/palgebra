#![allow(warnings)]

use std::io;
mod parser;
use parser::Parser;
mod token;
mod errors;
mod grammar;
mod ast_printer;

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
    io::stdin().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    expression
}

fn main() {
    welcome();
    let expression = read_expression();
    let mut parser = Parser::new();
    parser.scan(expression);
    // parser.print_tokens();
    if let Ok(expr) = parser.parse() {
        println!("{}", ast_printer::print(expr));
        println!("The proposition is a WFF");
    } else {
        println!("The proposition has errors; is not a WFF");
    }
}