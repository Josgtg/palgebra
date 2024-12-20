use crate::errors;
use crate::services::format::colored_bool;
use crate::token::Token;
use crate::types::*;
use crate::constants::ansi_codes::*;

pub fn print_possible(values: &Vec<LiteralAndBool>, idx: usize) -> String {
    let mut message = String::new();
    let mut colored: String;
    if !values.is_empty() {
        for v in &values[idx] {
            colored = colored_bool(v.1);
            message.push_str(&format!("{}{}: {} |{} ", GRAY, v.0, colored, RESET))
        }
        // removing the trailing " |", for some reason, since it's colored it takes 16 characters
        return String::from(&message[0..(message.len() - 15)]);
    }
    String::new()
}

pub fn replace_literals(
    tokens: &mut TokenSequence,
    close: bool,
) -> (Vec<TokenSequence>, Vec<LiteralAndBool>) {
    // Returns a vec with all possible values for every variable and another vec with those values
    // p & q -> [[True, And, True], [True, And, False], [False, And, True], [False, And, False]]
    //          [[(p, true), (q, true)], [(p, true), (q, false)], [(p, false), (q, true)] [...]]
    let mut counter: u32 = 0;
    for t in tokens.iter() {
        if let Token::Sentence(_) = t {
            counter += 1;
        }
    }
    if counter > 10 {
        if close {
            errors::fatal(
                "too many variables (more than 2048 or 2^11 lines would be printed), please replace some of the variables for literal values (true or false)",
                errors::Error::VarAmount,
                errors::codes::RUNTIME_ERROR
            );
        } else {
            errors::warn(
                "too many variables (more than 2048 or 2^11 lines would be printed), please replace some of the variables for literal values (true or false)",
                errors::Error::VarAmount
            );
        }
        return (Vec::new(), Vec::new());
    }

    if let Some(list) = transfrom_literals(tokens, &mut Vec::new()) {
        (list.0, list.1)
    } else {
        (vec![tokens.clone()], Vec::new())
    }
}

fn transfrom_literals(
    tokens: &mut TokenSequence,
    values: &mut LiteralAndBool,
) -> Option<(Vec<TokenSequence>, Vec<LiteralAndBool>)> {
    // Creates two vecs, one with the "true" variant of the variable, and other with the "false" variant.
    // It calls itself recursively untill all variables are replaced.

    let mut transform_result: Option<(Vec<TokenSequence>, Vec<LiteralAndBool>)>;
    let mut curr_char: char = '\0';

    let mut true_tree: Vec<TokenSequence>;
    let mut true_variant: TokenSequence = tokens.clone();
    let mut values_true_tree: Vec<LiteralAndBool>;
    let mut true_values: LiteralAndBool = values.clone();

    let mut false_tree: Vec<TokenSequence>;
    let mut false_variant: TokenSequence = tokens.clone();
    let mut values_false_tree: Vec<LiteralAndBool>;
    let mut false_values: LiteralAndBool = values.clone();

    let mut found_literal: bool = false;
    let mut i: usize = 0;
    for t in tokens {
        // Replaces every instance of a variable with true or false in its respective variant vec
        if let Token::Sentence(_) = t {
            if t.as_char() != curr_char && curr_char != '\0' {
                i += 1;
                continue;
            }
            curr_char = t.as_char();
            true_variant[i] = Token::True;
            false_variant[i] = Token::False;
            found_literal = true;
        }
        i += 1;
    }

    if !found_literal {
        return None;
    }

    true_values.push((curr_char, true));
    transform_result = transfrom_literals(&mut true_variant, &mut true_values);
    // We call this function recursively with the vec that has already replace the variable for a "true", in this case
    if transform_result.is_none() {
        // No more variables to replace, so we return the current vector
        true_tree = vec![true_variant];
        values_true_tree = vec![true_values];
    } else {
        (true_tree, values_true_tree) = transform_result.unwrap();
    }

    false_values.push((curr_char, false));
    transform_result = transfrom_literals(&mut false_variant, &mut false_values);
    if transform_result.is_none() {
        false_tree = vec![false_variant];
        values_false_tree = vec![false_values];
    } else {
        (false_tree, values_false_tree) = transform_result.unwrap();
    }

    true_tree.append(&mut false_tree);
    values_true_tree.append(&mut values_false_tree);
    Some((true_tree, values_true_tree))
}
