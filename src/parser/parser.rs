// use std::{collections::HashSet};

use super::scanner;
use crate::{errors, grammar::Expr, token::Token};

fn is_operator_token(token: &Token) -> bool {
    token == &Token::And || token == &Token::Or || token == &Token::IfOnlyIf || token == &Token::IfThen || token == &Token::Not
}

pub struct Parser {
    // pub sentences: HashSet<char>,
    pub tokens: Vec<Token>,
    pub error: bool,
    open_parenthesis: u32,
    start_idx: usize,
    idx: usize
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            // sentences: HashSet::new(),
            tokens: Vec::new(),
            error: false,
            open_parenthesis: 0,
            start_idx: 0,
            idx: 0
        }
    }

    pub fn scan(&mut self, proposition: &str) {
       scanner::scan(self, proposition);
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ()> {
        let proposition = self.proposition();

        while !self.is_at_end() {
            self.error = true;
            let proposition = self.proposition();
        }

        if self.error { Err(()) }
        else { Ok(Box::new(proposition)) }
    }

    // Building the tree

    fn proposition(&mut self, )  -> Expr {
        self.start_idx = self.idx;
        let mut proposition = self.unary();

        while self.match_tokens(vec![Token::And, Token::Or, Token::IfOnlyIf, Token::IfThen]) {
            if proposition == Expr::Null {
                self.error("missing proposition on left side of operation");
                continue;
            }

            self.start_idx = self.idx;
            
            let operator = self.previous_owned();

            let mut rigth = self.unary();

            if rigth == Expr::Null {
                if is_operator_token(self.peek()) {
                    self.error("operators are next to each other");
                    continue;
                }

                if self.peek() == &Token::RightParen {
                    if self.open_parenthesis > 0 {
                        self.open_parenthesis -= 1;
                    } else {
                        self.error("unmatched closing parenthesis");
                        continue;
                    }
                }
                if self.match_token(Token::Invalid) {
                    rigth = self.unary();
                } else {
                    self.error("missing proposition on right side of operation");
                    continue;
                }
            }

            proposition = Expr::Binary(Box::new(proposition), operator, Box::new(rigth))
        }

        if self.match_token(Token::Invalid) {
            proposition = self.proposition();
        }

        if let Token::Sentence(_) = self.peek() {
            proposition = self.unary();
        }

        if self.peek() == &Token::Not {
            self.manual_error("not operator is in an invalid position", 0, self.idx);
        }

        if self.peek() == &Token::LeftParen {
            proposition = self.unary();
        }

        proposition
    }

    fn unary(&mut self) -> Expr {
        self.start_idx = self.idx;

        if self.match_token(Token::Not) {
            let mut right = self.proposition();
            if right == Expr::Null {
                self.error("missing proposition on right side of negation");
                return Expr::Null;
            }
            return Expr::Unary(Token::Not, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        self.start_idx = self.idx;

        if self.match_token(Token::LeftParen) {
            self.open_parenthesis += 1;
            let proposition = self.proposition();
            if self.open_parenthesis > 0 {
                if self.match_token(Token::RightParen) {
                    self.open_parenthesis -= 1;
                } else {
                    self.manual_error("expected closing parenthesis", 0, self.idx);
                }
            }
            if proposition == Expr::Null {
                if !self.is_at_end() { self.manual_error("not a proposition", 1, self.start_idx); }
                return Expr::Null;
            }
            return Expr::Grouping(Box::new(proposition))
        }

        if let Token::Sentence(_) = self.peek() {
            Expr::Literal(self.advance_owned())
        } else {
            if self.match_token(Token::RightParen) {
                if self.open_parenthesis > 0 {
                    self.open_parenthesis -= 1;
                } else {
                    self.manual_error("closing parenthesis does not have a match", 0, self.idx - 1);
                }
            }
            Expr::Null
        }
    }

    // Help
    
    fn is_at_end(&self) -> bool {
        self.idx >= self.tokens.len()
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.peek() == &token {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, tokens: Vec<Token>) -> bool {
        for token in tokens {
            if self.match_token(token) {
                return true;
            }
        }
        false
    }

    fn expect(&mut self, to_compare: Token, fail_message: &str) -> bool {
        if self.is_at_end() {
            self.error(&format!("{}, but proposition finished", fail_message));
            return false;
        }
        if self.peek() != &to_compare {
            self.error(fail_message);
            return false;
        }
        self.advance();
        true
    }

    fn close_parenthesis(&mut self, error: &str) -> bool {
        if self.match_token(Token::RightParen) {
            if self.open_parenthesis > 0 {
                self.open_parenthesis -= 1;
                return true;
            }
        }
        self.manual_error(error, 0, self.idx);
        false
    }

    // Error handling

    fn error(&mut self, message: &str) {
        self.error = true;
        errors::report(message, 0, (self.start_idx + 1) as u32);
        self.synchronize();
    }

    fn manual_error(&mut self, message: &str, code: u32, idx: usize) {
        self.error = true;
        errors::report(message, code, (idx + 1) as u32);
        self.synchronize();
    }

    fn synchronize(&mut self) {
        /*
        When there is an error, we need to get to a point where we can continue catching
        errors without being affected by the previous ones. That point is either in a new sentence
        or a left prenthesis.
        */
        while !self.is_at_end() {
            if let Token::Sentence(_) = self.peek() {
                return
            }
            if self.peek() == &Token::LeftParen {
                return
            }
            if self.peek() == &Token::RightParen {
                if self.open_parenthesis > 0 { self.open_parenthesis -= 1; }
            }
            self.advance();
        }
    }

    // Token consuming
    
    fn previous(&self) -> &Token {
        &self.tokens[self.idx - 1]
    }

    fn previous_owned(&self) -> Token {
        self.tokens[self.idx - 1].clone()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            return &Token::Null;
        }
        &self.tokens[self.idx]
    }

    fn peek_owned(&self) -> Token {
        if self.is_at_end() {
            return Token::Null;
        }
        self.tokens[self.idx].clone()
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return &Token::Null;
        }
        self.idx += 1;
        &self.tokens[self.idx - 1]
    }

    fn advance_owned(&mut self) -> Token {
        if self.is_at_end() {
            return Token::Null;
        }
        self.idx += 1;
        self.tokens[self.idx - 1].clone()
    }

    fn next(&mut self) -> &Token {
        self.idx += 1;
        if self.is_at_end() {
            return &Token::Null;
        }
        self.idx -= 1;
        &self.tokens[self.idx + 1]
    }

    fn next_owned(&mut self) -> Token {
        self.idx += 1;
        if self.is_at_end() {
            return Token::Null;
        }
        self.idx -= 1;
        self.tokens[self.idx + 1].clone()
    }

    // Debugging

    pub fn print_tokens(&self) {
        println!("{:?}", self.tokens);
    }
}
