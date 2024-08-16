use super::Parser;
use crate::token::Token;
use crate::errors;

pub fn scan(parser: &mut Parser, proposition: String) {
    let mut index: u32 = 1;
    for c in proposition.chars() {
        if ignore(c) {
            index += 1;
            continue;
        }
        match_token(c, parser, index);
        index += 1;
    }
}

fn match_token(c: char, parser: &mut Parser, index: u32) {
    match c {
        '&' => parser.tokens.push(Token::And),
        '|' => parser.tokens.push(Token::Or),
        '!' => parser.tokens.push(Token::Not),
        '~' => parser.tokens.push(Token::IfOnlyIf),
        '>' => parser.tokens.push(Token::IfThen),
        '(' => parser.tokens.push(Token::LeftParen),
        ')' => parser.tokens.push(Token::RightParen),
        _ => {
            if !c.is_alphabetic() {
                parser.error = true;
                errors::report(&format!("unexpected character \"{}\"", c), 0, index);
                parser.tokens.push(Token::Invalid);
                return
            }
            parser.sentences.insert(c);
            parser.tokens.push(Token::Sentence(c))
        }
    }
}

fn ignore(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\0' || c == '\n' || c == '\r'
}
