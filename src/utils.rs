use crate::token::Token;
use crate::errors;
use std::{fs, vec};
use std::{collections::{HashMap, HashSet}, io::{self, BufRead, Write}};

pub fn colorize(i: u32, b: bool) {
    if i == 0 {
        if b {
            println!("\x1b[92m{}\x1b[0m", b);
        } else {
            println!("\x1b[31m{}\x1b[0m", b);
        }
    } else {
        if b {
            println!("{}: \x1b[92m{}\x1b[0m", i, b);
        } else {
            println!("{}: \x1b[31m{}\x1b[0m", i, b);
        }
    }
}

pub fn read_expression_from_file(path: &str) -> String {
    let proposition = fs::read_to_string(path);
    if let Ok(s) = proposition {
        return s;
    }
    errors::fatal("file could not be read", 3, 1);
    String::new()
}

pub fn read_expression_from_user() -> String {
    print!(">>> ");
    io::stdout().flush();
    let mut expression = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut expression).expect("Failed to read line. Restart the program and try again");
    String::from(expression.trim())
}

pub fn divide_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut vec = Vec::new();
    let mut curr: Vec<Token> = Vec::new();
    for t in tokens {
        if t == Token::NewLine || t == Token::Comment {
            vec.push(curr);
            curr = Vec::new();
            continue;
        }
        curr.push(t);
    }
    if !curr.is_empty() {
        vec.push(curr);
    }
    vec
}

pub fn print_possible(values: &Option<Vec<Vec<(char, bool)>>>, idx: usize) -> String {
    let mut message = String::new();
    let mut colored: &str;
    if let Some(vv) = values {
        for v in &vv[idx] {
            if v.1 {
                colored = "\x1b[92mtrue \x1b[0m";
            } else {
                colored = "\x1b[31mfalse\x1b[0m";
            }
            message.push_str(&format!("{}: {} \x1b[90m|\x1b[0m ", v.0, colored))
        }

        return message;
    }
    String::new()
}

pub fn replace_literals(tokens: &mut Vec<Token>) -> (Vec<Vec<Token>>, Option<Vec<Vec<(char, bool)>>>) {
    let mut counter: u32 = 0;
    for t in tokens.iter() {
        if let Token::Sentence(_) = t { counter += 1; }
    }
    if counter > 10 {
        errors::fatal("too many variables (more than 2048 or 2^12 lines would be printed), please replace some of the variables for literal values (true or false)", 3, 1);
        return (Vec::new(), None)
    }

    if let Some(list) = transfrom_literals(tokens, &mut Vec::new()) {
        (list.0, Some(list.1))
    } else {
        (vec![tokens.clone()], None)
    }
}

fn transfrom_literals(tokens: &mut Vec<Token>, values: &mut Vec<(char, bool)>) -> Option<(Vec<Vec<Token>>, Vec<Vec<(char, bool)>>)> {
    let mut transform_result: Option<(Vec<Vec<Token>>, Vec<Vec<(char, bool)>>)>;
    let mut curr_char: char = '\0';

    let mut vec_true_tree: Vec<Vec<Token>> = Vec::new();
    let mut vec_true: Vec<Token> = tokens.clone();
    let mut values_true_tree: Vec<Vec<(char, bool)>> = Vec::new();
    let mut values_true: Vec<(char, bool)> = values.clone();

    let mut vec_false_tree: Vec<Vec<Token>> = Vec::new();
    let mut vec_false: Vec<Token> = tokens.clone();
    let mut values_false_tree: Vec<Vec<(char, bool)>> = Vec::new();
    let mut values_false: Vec<(char, bool)> = values.clone();

    let mut found_literal: bool = false;
    let mut i: usize = 0;
    for t in tokens {
        if let Token::Sentence(_) = t {
            if t.as_char() != curr_char && curr_char != '\0' {
                i += 1;
                continue;
            }
            curr_char = t.as_char();
            vec_true[i] = Token::True;
            vec_false[i] = Token::False;
            found_literal = true;
        }
        i += 1;
    }

    if !found_literal { return None }

    values_true.push((curr_char, true));
    transform_result = transfrom_literals(&mut vec_true, &mut values_true);
    if let None = transform_result {
        vec_true_tree = vec![vec_true];
        values_true_tree = vec![values_true];
    } else {
        (vec_true_tree, values_true_tree) = transform_result.unwrap();
    }

    values_false.push((curr_char, false));
    transform_result = transfrom_literals(&mut vec_false, &mut values_false);
    if let None = transform_result {
        vec_false_tree = vec![vec_false];
        values_false_tree = vec![values_false];
    } else {
        (vec_false_tree, values_false_tree) = transform_result.unwrap();
    }

    vec_true_tree.append(&mut vec_false_tree);
    values_true_tree.append(&mut values_false_tree);
    Some((vec_true_tree, values_true_tree))
}
