use crate::token::Token;
use crate::errors;
use std::collections::HashSet;

pub struct Parser {
    sentences: HashSet<char>,
    token_list: Vec<Token>,
    idx: u32
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            sentences: HashSet::new(),
            token_list: Vec::new(),
            idx: 1
        }
    }

    pub fn scan(&mut self, expression: String) -> Vec<Token> {
        for c in expression.chars() {
            if self.ignore(c) {
                self.idx += 1;
                continue;
            }
            self.match_token(c);
            self.idx += 1;
        }
        println!("{:?}", self.sentences);
        return self.token_list.clone()
    }

    fn match_token(&mut self, c: char) {
        match c {
            '&' => self.token_list.push(Token::And),
            '|' => self.token_list.push(Token::Or),
            '!' => self.token_list.push(Token::Not),
            '~' => self.token_list.push(Token::IfOnlyIf),
            '>' => self.token_list.push(Token::IfThen),
            '(' => self.token_list.push(Token::LeftParen),
            ')' => self.token_list.push(Token::RightParen),
            _ => {
                if !c.is_alphabetic() {
                    errors::report(&format!("unexpected character \"{}\"", c), 0, self.idx);
                    return
                }
                self.sentences.insert(c);
                self.token_list.push(Token::Sentence(c))
            }
        }
    }

    fn ignore(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\0' || c == '\n' || c == '\r'
    }
}
