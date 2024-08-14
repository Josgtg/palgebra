use std::io;
mod parser;
use parser::Parser;
mod token;
mod errors;

fn welcome() {
    let mut message: String = String::new();
    message += "Propositional algebra evaluator\n";
    message += "Write a proposition and it will be evaluated to know if it is a well formed formula.\n";
    message += "Symbol list:\n";
    message += "and: &\nor: |\nnot: !\nif and only if: ~\nif, then: >\n";
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
}