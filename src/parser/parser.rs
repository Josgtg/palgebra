use std::{collections::HashSet};

use super::scanner;
use crate::{ast_printer, errors, grammar::Expr, token::Token};
pub struct Parser {
    pub sentences: HashSet<char>,
    pub tokens: Vec<Token>,
    pub error: bool,
    idx: usize
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            sentences: HashSet::new(),
            tokens: Vec::new(),
            error: false,
            idx: 0
        }
    }

    pub fn scan(&mut self, proposition: String) {
       scanner::scan(self, proposition);
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ()> {
        let expr = self.expression();
        if self.error {
            return Err(());
        }
        Ok(expr)
    }

    // Actual parsing

    fn expression(&mut self) -> Box<Expr> {
        let mut expr = self.unary();
        if self.is_at_end() {
            return expr;
        }
        if *expr == Expr::InvalidToken {
            self.synchronize();
            expr = self.expression();
        }
        if let Expr::Operation(_) = *expr  {
            self.error("missing expression on left side of operand");
        } 
        if *expr == Expr::Null {
            self.error("")
        }

        let mut op: Token;
        let mut right: Box<Expr>;
        while self.match_tokens(vec![Token::And, Token::Or, Token::IfOnlyIf, Token::IfThen]) {
            op = self.previous_owned();
            right = self.unary();
            if *right == Expr::Null {
                self.error("missing expression on right side of operand");
                continue;
            }
            if let Expr::Operation(_) = *right {
                self.error("operators are next to each other");
                continue;
            }
            expr = Box::new(Expr::Binary(expr, op, right));
        }

        if let Token::Sentence(_) = self.peek() {
            self.error("sentence is in an invalid position");
            expr = self.expression();
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(Token::Not) {
            let op = self.previous_owned();
            let right = self.unary();
            if *right == Expr::Null {
                self.error("missing expression on right side of negation");
            }
            return Box::new(Expr::Unary(op, right))
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_token(Token::LeftParen) {
            let expr = self.expression();
            self.expect(Token::RightParen, "expected closing parenthesis");
            return Box::new(Expr::Grouping(expr));
        }

        let token = self.peek();
        if let Token::Sentence(c) = token {
            return Box::new(Expr::Literal(self.advance_owned()));
        }
        
        if token == &Token::RightParen {
            self.error("closing parenthesis does not have a match");
            return self.expression();
        }

        if self.is_operator(self.peek()) {
            return Box::new(Expr::Operation(self.advance_owned()));
        }

        if self.peek() == &Token::Invalid {
            return Box::new(Expr::InvalidToken);
        }
        
        Box::new(Expr::Null)
    }

    // Help
    
    fn is_at_end(&self) -> bool {
        self.idx >= self.tokens.len()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.idx - 1]
    }

    fn previous_owned(&self) -> Token {
        self.tokens[self.idx - 1].clone()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            return &Token::Null
        }
        &self.tokens[self.idx]
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

    fn match_token(&mut self, t: Token) -> bool {
        if self.peek() == &t {
            self.advance();
            return true
        }
        false
    }

    fn match_tokens(&mut self, tokens: Vec<Token>) -> bool {
        for t in tokens {
            if self.peek() == &t {
                self.advance();
                return true
            }
        }
        false
    }

    fn error(&mut self, message: &str) {
        errors::report(message, 1, (self.idx + 1) as u32);
        self.error = true;
        self.synchronize();
    }

    fn expect(&mut self, to_match: Token, failure_message: &str) -> bool {
        if self.is_at_end() {
            self.error(&(String::from(failure_message) + ", but proposition finished"));
            return false;
        } else if self.peek() != &to_match {
            self.error(&format!("{}, got \"{}\"", failure_message, self.peek().as_char()));
            return false;
        }
        self.advance();
        true
    }

    fn is_operator(&self, token: &Token) -> bool {
        token == &Token::And || token == &Token::Or || token == &Token::IfOnlyIf || token == &Token::IfThen
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if let Token::Sentence(_) = self.peek() {
                return
            } else if self.peek() == &Token::LeftParen {
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
