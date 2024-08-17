#![allow(warnings)]

use std::io::{self, BufRead};
mod parser;
use parser::Parser;
mod token;
mod errors;
mod grammar;
use grammar::Expr;
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
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    expression
}

pub fn parse(proposition: String) -> Result<Box<Expr>, ()> {
    let mut parser = Parser::new();
    parser.scan(proposition);
    // parser.print_tokens();
    parser.parse()
}
fn main() {
    welcome();

    let expression = read_expression();
    let res = parse(expression);
    println!();

    if let Ok(expr) = res {
        println!("{}", ast_printer::print(expr));
        println!("Good! The proposition is a WFF");
    } else {
        println!("The proposition has errors; is not a WFF");
    }
}