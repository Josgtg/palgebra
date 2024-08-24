mod tests;
mod interpreter;
mod scanner;
mod parser;
mod token;
mod errors;
mod grammar;
mod utils;
mod possible;

use utils::*;
use possible::*;
use token::Token;

fn welcome() {
    let mut message: String = String::new();
    message += "----------------------------\nSymbol list:\n";
    message += "and: &\nor: |\nnot: !\nif and only if: ~\nif, then: >\n----------------------------\n";
    message += "0 and 1 or \"false\" and \"true\" respectively are considered literal simple propositions\n";
    message += "Any alphabetical letter will be interpreted as a variable simple proposition and all its possible values will be evaluated\n";
    message += "You can group using parenthesis\n";
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
    println!("usage: plogic <file_name>\nIf no file is given, an interactive session will start");
    std::process::exit(64);
}

fn interactive() {
    let mut err: bool;
    let mut expr: Box<grammar::Expr>;
    let mut variant_num: usize;
    let mut possible: String;
    let mut proposition: String;
    let mut tokens: Vec<token::Token>;
    let mut i: usize = 1;

    welcome();
    loop {
        err = false;
        proposition = read_expression_from_user();

        if proposition.is_empty() { continue }
        else if proposition.eq("exit") { return }
        else if proposition.eq("cls") || proposition.eq("clear") {
            // clear screen and place the cursor in row 1 col 1
            print!("{esc}c{esc}[1;1H", esc = 27 as char);
        }

        let res_scan_tokens= scanner::scan(&proposition, i as u32);
        if res_scan_tokens.is_err() {
            err = true;
            tokens = res_scan_tokens.unwrap_err();
        } else {
            tokens = res_scan_tokens.unwrap();
        }

        if tokens[tokens.len() - 1] == Token::Comment {
            tokens.pop();
            if tokens.len() == 0 { continue }
        }

        if parser::parse(tokens.clone(), i as u32).is_err() { continue }
        if err { continue }

        variant_num = 0;
        let (t, values) = replace_literals(&mut tokens, true);
        for variant in t {
            expr = parser::parse(variant, i as u32).unwrap();
            possible = print_possible(&values, variant_num);
            if !possible.is_empty() { println!("{}", possible); }
            colorize(interpreter::interpret(*expr));
            variant_num += 1;
        }
        i += 1;
    }
}

fn from_file(path: &str) {
    let mut err: bool;
    let mut expr: Box<grammar::Expr>;
    let mut possible: String;
    let mut variant_num: usize;
    let mut scan_tokens: Vec<token::Token>;

    let whole_proposition = utils::read_expression_from_file(path);
    
    let divided = divide_proposition(whole_proposition);

    for (i, proposition) in divided.into_iter().enumerate() {
        println!("Proposition: {}", &proposition);
        err = false;
        let res_scan_tokens= scanner::scan(&proposition, 1);
        if let Err(_) = res_scan_tokens {
            err = true;
            scan_tokens = res_scan_tokens.unwrap_err();
        } else {
            scan_tokens = res_scan_tokens.unwrap();
        }
        if parser::parse(scan_tokens.clone(), (i + 1) as u32).is_err() { println!(); continue; }
        if err { continue }

        let (t, values) = replace_literals(&mut scan_tokens, false);
        variant_num = 0;
        for variant in t {
            expr = parser::parse(variant, (i + 1) as u32).unwrap();
            possible = print_possible(&values, variant_num);
            if !possible.is_empty() { println!("{}\x1b[0m", possible); }
            colorize(interpreter::interpret(*expr));
            variant_num += 1;
        }
        println!();
    }
}

fn _test(proposition: &str) {
    let mut err: bool;
    let mut variant_num: usize;
    let mut tokens: Vec<token::Token>;
    err = false;

    if proposition.is_empty() || proposition.eq("exit") { return }

    let res_scan_tokens= scanner::scan(&proposition, 1);
    if res_scan_tokens.is_err() {
        err = true;
        tokens = res_scan_tokens.unwrap_err();
    } else {
        tokens = res_scan_tokens.unwrap();
    }

    if tokens[tokens.len() - 1] == Token::Comment {
        tokens.pop();
        if tokens.len() == 0 { return }
    }

    if parser::parse(tokens.clone(), 1).is_err() { return }
    if err { return }

    variant_num = 0;
    let (t, values) = replace_literals(&mut tokens, false);
    for tt in &t {
        println!("{:?}", tt);
    }
    for variant in t {
        println!("{}", variant_num);
        if let Ok(expr) = parser::parse(variant, 1) {
            println!("{}", print_possible(&values, variant_num));
            colorize(interpreter::interpret(*expr));
        }
        variant_num += 1;
    }
}
