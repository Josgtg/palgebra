use crate::errors;
use crate::token::Token;

pub fn print_possible(values: &Option<Vec<Vec<(char, bool)>>>, idx: usize) -> String {
    let mut message = String::new();
    let mut colored: &str;
    if let Some(vv) = values {
        for v in &vv[idx] {
            if v.1 {
                colored = "\x1b[92mtrue \x1b[0m";
            } else {
                colored = "\x1b[91mfalse\x1b[0m";
            }
            message.push_str(&format!("{}: {} \x1b[2m|\x1b[0m ", v.0, colored))
        }
        // removing the trailing " |", for some reason, since it's colored it takes 16 characters
        return String::from(&message[0..(message.len() - 15)]);
    }
    String::new()
}

pub fn replace_literals(tokens: &mut Vec<Token>, close: bool) -> (Vec<Vec<Token>>, Option<Vec<Vec<(char, bool)>>>) {
    // Returns a vec with all possible values for every variable and another vec with those values
    // p & q -> [[True, And, True], [True, And, False], [False, And, True], [False, And, False]]
    //          [[(p, true), (q, true)]. [(p, true), (q, false)], [(p, false), (q, true)] [...]]
    let mut counter: u32 = 0;
    for t in tokens.iter() {
        if let Token::Sentence(_) = t { counter += 1; }
    }
    if counter > 10 {
        errors::fatal("too many variables (more than 2048 or 2^11 lines would be printed), please replace some of the variables for literal values (true or false)", 3, 1, !close);
        return (Vec::new(), None)
    }

    if let Some(list) = transfrom_literals(tokens, &mut Vec::new()) {
        (list.0, Some(list.1))
    } else {
        (vec![tokens.clone()], None)
    }
}

fn transfrom_literals(tokens: &mut Vec<Token>, values: &mut Vec<(char, bool)>) -> Option<(Vec<Vec<Token>>, Vec<Vec<(char, bool)>>)> {
    // Creates two vecs, one with the "true" variant of the variable, and other with the "false" variant.
    // It calls itself recursively untill all variables are replaced.
    
    let mut transform_result: Option<(Vec<Vec<Token>>, Vec<Vec<(char, bool)>>)>;
    let mut curr_char: char = '\0';

    let mut true_tree: Vec<Vec<Token>>;
    let mut true_variant: Vec<Token> = tokens.clone();
    let mut values_true_tree: Vec<Vec<(char, bool)>>;
    let mut true_values: Vec<(char, bool)> = values.clone();

    let mut false_tree: Vec<Vec<Token>>;
    let mut false_variant: Vec<Token> = tokens.clone();
    let mut values_false_tree: Vec<Vec<(char, bool)>>;
    let mut false_values: Vec<(char, bool)> = values.clone();

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

    if !found_literal { return None }

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
