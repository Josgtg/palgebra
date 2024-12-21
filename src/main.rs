mod cli;
mod constants;
mod errors;
mod grammar;
mod interpreter;
mod parser;
mod scanner;
mod services;
mod tests;
mod token;
mod types;
mod simplifier;

use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use errors::codes;
use grammar::Expr;
use services::reader;
use types::TokenSequence;

fn main() {
    let args = Cli::parse();
    if let Some(path) = args.read_path {
        from_file(path);
    } else {
        interactive();
    }
}

fn interactive() {
    let mut line: u32 = 1;
    let mut input: String;
    let mut tokens: TokenSequence;
    let mut expression: Expr;

    loop {
        input = reader::read_expression_from_user();
        if input.is_empty() {
            line += 1;
            continue;
        } else if input.eq("exit") {
            return;
        }

        if let Ok(t) = scanner::scan(&input, line) {
            tokens = t;
        } else {
            errors::warn("proposition is not readable", errors::Error::InvalidProposition);
            continue;
        }
        
        
        if let Ok(e) = parser::parse(tokens, line) {
            expression = simplifier::simplify(e);
        } else {
            errors::warn("proposition is not well formed", errors::Error::InvalidLogic);
            continue;
        }

        println!("{}", expression);

        line += 1;
    }
}


fn from_file(path: PathBuf) {
    let line: u32 = 1;
    let input: String;
    let tokens: TokenSequence;
    let expression: Expr;

    input = reader::read_expression_from_file(path);

    if let Ok(t) = scanner::scan(&input, line) {
        tokens = t;
    } else {
        errors::fatal("proposition is not readable", errors::Error::InvalidProposition, codes::RUNTIME_ERROR);
        return;
    }
    
    if let Ok(e) = parser::parse(tokens, line) {
        expression = simplifier::simplify(e);
    } else {
        errors::fatal("proposition is not well formed", errors::Error::InvalidLogic, codes::RUNTIME_ERROR);
        return;
    }

    println!("{}", expression);
}
