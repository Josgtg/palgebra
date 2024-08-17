use std::{collections::HashSet};

use super::scanner;
use crate::{ast_printer, errors, grammar::Expr, token::Token};
pub struct Parser {
    pub sentences: HashSet<char>,
    pub tokens: Vec<Token>,
    pub error: bool,
    start_idx: usize,
    idx: usize
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            sentences: HashSet::new(),
            tokens: Vec::new(),
            error: false,
            start_idx: 0,
            idx: 0
        }
    }

    pub fn scan(&mut self, proposition: &str) {
       scanner::scan(self, proposition);
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ()> {
        let proposition = self.proposition();
        if !self.is_at_end() {
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
            self.start_idx = self.idx;
            let operator = self.previous_owned();
            let rigth = self.unary();
            proposition = Expr::Binary(Box::new(proposition), operator, Box::new(rigth))
        }

        proposition
    }

    fn unary(&mut self) -> Expr {
        self.start_idx = self.idx;

        if self.match_token(Token::Not) {
            let right = self.proposition();
            return Expr::Unary(Token::Not, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        self.start_idx = self.idx;

        if self.match_token(Token::LeftParen) {
            let proposition = self.proposition();
            self.expect(Token::RightParen, "expected closing parenthesis");
            return Expr::Grouping(Box::new(proposition))
        }

        Expr::Literal(self.advance_owned())
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
        self.idx += 1;
        if self.is_at_end() {
            return &Token::Null;
        }
        &self.tokens[self.idx - 1]
    }

    fn advance_owned(&mut self) -> Token {
        self.idx += 1;
        if self.is_at_end() {
            return Token::Null;
        }
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

    // Error handling

    fn error(&mut self, message: &str) {
        self.error = true;
        errors::report(message, 1, self.start_idx as u32);
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
                self.start_idx = self.idx;
                return
            }
            if self.peek() == &Token::LeftParen {
                self.start_idx = self.idx;
                return
            }
            self.advance();
        }
    }

    // Debugging

    pub fn print_tokens(&self) {
        println!("{:?}", self.tokens);
    }
}
