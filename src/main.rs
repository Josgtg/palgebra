#![allow(warnings)]

mod tests;
mod interpreter;
mod scanner;
mod parser;
mod token;
mod errors;
mod grammar;
mod utils;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::Write;
use utils::*;
use token::Token;
use grammar::Expr;

fn welcome_() {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        interactive();
        return
    }
    if args.len() == 2 {
        from_file(&args[1]);
        return
    }
    println!("usage: paleval <file_name>\nIf no file is specificated, an interactive session will start");
    std::process::exit(64);
}

fn interactive() {
    let mut err: bool;
    let mut expr: Box<grammar::Expr>;
    let mut variant_num: usize;
    let mut possible: String;
    let mut proposition: String;
    let mut values: Vec<char>;
    let mut tokens: Vec<token::Token>;
    let mut i: usize = 1;
    loop {
        err = false;
        proposition = read_expression_from_user();

        if proposition.is_empty() { continue }
        else if proposition.eq("exit") { return }

        let res_scan_tokens= scanner::scan(&proposition, 1);
        if let Err(_) = res_scan_tokens {
            err = true;
            tokens = res_scan_tokens.unwrap_err();
        } else {
            tokens = res_scan_tokens.unwrap();
        }

        if let Err(_) = parser::parse(tokens.clone(), i as u32) { continue; }
        if err { continue; }

        variant_num = 0;
        let (t, values) = replace_literals(&mut tokens);
        for variant in t {
            expr = parser::parse(variant, i as u32).unwrap();
            possible = print_possible(&values, variant_num);
            if !possible.is_empty() { println!("{}", possible); }
            colorize(0, interpreter::interpret(expr));
            i += 1;
            variant_num += 1;
        }
    }
}

fn from_file(path: &str) {
    let mut expr: Box<grammar::Expr>;
    let mut possible: String;
    let mut variant_num: usize;
    let mut err: bool = false;
    let mut scan_tokens: Vec<token::Token>;

    let proposition = utils::read_expression_from_file(path);

    let res_scan_tokens= scanner::scan(&proposition, 1);
    if let Err(_) = res_scan_tokens {
        err = true;
        scan_tokens = res_scan_tokens.unwrap_err();
    } else {
        scan_tokens = res_scan_tokens.unwrap();
    }

    let divided = divide_tokens(scan_tokens);
    let len_divided = divided.len();
    let mut counter_counter: u32 = 0;
    for (i, tokens) in divided.iter().enumerate() {
        let mut counter: u32 = 0;
        for t in tokens.iter() {
            if let Token::Sentence(_) = t { counter += 1; }
        }

        counter_counter += counter;
    }

    if (len_divided * 2^counter_counter as usize) > 1024 {
        errors::fatal("too many variables (more than 2048 or 2^12 lines would be printed), please replace some of the variables for literal values (true or false)", 3, 1);
        return
    }

    for (i, mut tokens) in divided.into_iter().enumerate() {
        if let Err(_) = parser::parse(tokens.clone(), (i + 1) as u32) { continue; }
        if err { continue }

        let (t, values) = replace_literals(&mut tokens);
        variant_num = 0;
        for variant in t {
            expr = parser::parse(variant, (i + 1) as u32).unwrap();
            possible = print_possible(&values, variant_num);
            if !possible.is_empty() { println!("{}", possible); }
            colorize((i + 1) as u32, interpreter::interpret(expr));
            variant_num += 1;
        }
    }
}

fn test(proposition: &str) {
    let mut err: bool;
    let mut variant_num: usize;
    let mut values: Vec<char>;
    let mut tokens: Vec<token::Token>;
    let mut i: usize = 1;
    err = false;

    if proposition.is_empty() { return }
    else if proposition.eq("exit") { return }

    let res_scan_tokens= scanner::scan(&proposition, 1);
    if let Err(_) = res_scan_tokens {
        err = true;
        tokens = res_scan_tokens.unwrap_err();
    } else {
        tokens = res_scan_tokens.unwrap();
    }

    if let Err(_) = parser::parse(tokens.clone(), i as u32) { return }
    if err { return }

    variant_num = 0;
    let (t, values) = replace_literals(&mut tokens);
    for tt in &t {
        println!("{:?}", tt);
    }
    for variant in t {
        println!("{}", variant_num);
        if let Ok(expr) = parser::parse(variant, i as u32) {
            println!("{}", print_possible(&values, variant_num));
            colorize(0, interpreter::interpret(expr));
            i += 1;
        }
        variant_num += 1;
    }
}
