use crate::{
    errors::{
        self,
        Error::{self, *},
    }, grammar::Expr, token::Token, types::TokenSequence
};

pub fn parse(tokens: TokenSequence, line: u32) -> Result<Expr, ()> {
    let mut parser = Parser::new(tokens, line);
    parser.parse()
}

fn is_literal_value_token(token: &Token) -> bool {
    if let Token::Sentence(_) = token {
        return true;
    }
    token == &Token::True || token == &Token::False
}

fn is_operator_token(token: &Token) -> bool {
    token == &Token::And
        || token == &Token::Or
        || token == &Token::IfOnlyIf
        || token == &Token::IfThen
        || token == &Token::Not
}

pub struct Parser {
    pub tokens: TokenSequence,
    pub error: bool,
    open_parenthesis: u32,
    line: u32,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: TokenSequence, line: u32) -> Self {
        Parser {
            tokens,
            error: false,
            open_parenthesis: 0,
            line,
            idx: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        let expression = self.expression();
        while !self.is_at_end() {
            self.error = true;
            self.expression();
        }

        if self.error {
            Err(())
        } else {
            Ok(expression)
        }
    }

    // Building the tree

    fn expression(&mut self) -> Expr {
        let mut expression = self.unary();

        let mut start_idx = self.idx;

        while self.match_tokens(&[Token::And, Token::Or, Token::IfOnlyIf, Token::IfThen]) {
            if expression.is_null() {
                self.error(
                    "missing expression on left side of operation",
                    SyntaxError,
                    start_idx,
                );
                continue;
            }

            start_idx = self.idx;

            let operator = self.previous_owned();

            let mut right = self.unary();

            if right.is_null() {
                if is_operator_token(self.peek()) {
                    self.error("operators are next to each other", SyntaxError, self.idx);
                    continue;
                }

                if self.peek() == &Token::RightParen {
                    if self.open_parenthesis > 0 {
                        self.open_parenthesis -= 1;
                    } else {
                        self.error("unmatched closing parenthesis", SyntaxError, self.idx);
                        continue;
                    }
                }

                if self.match_token(&Token::Invalid) {
                    self.error = true;
                    right = self.unary();  // right
                } else {
                    self.error(
                        "missing expression on right side of operation",
                        SyntaxError,
                        start_idx,
                    );
                    continue;
                }
            }

            expression = Expr::binary(expression, operator, right);
        }

        if is_literal_value_token(self.peek()) {
            self.error = true;
            expression = self.expression();
        }

        if self.match_token(&Token::Invalid) || self.peek() == &Token::LeftParen {
            self.error = true;
            expression = self.expression();
        }

        if self.peek() == &Token::Not {
            self.error(
                "not operator is in an invalid position",
                SyntaxError,
                self.idx,
            );
        }

        expression
    }

    fn unary(&mut self) -> Expr {
        let start_idx = self.idx;

        if self.match_token(&Token::Not) {
            let right = self.unary();
            if right.is_null() {
                self.error(
                    "missing expression on right side of negation",
                    SyntaxError,
                    start_idx,
                );
            }
            return Expr::unary(Token::Not, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let start_idx = self.idx;

        if is_literal_value_token(self.previous()) {
            if self.peek() == &Token::LeftParen {
                self.error("grouping in invalid position", SyntaxError, start_idx);
            }
            if is_literal_value_token(self.peek()) {
                self.error(
                    "simple expression is in an invalid position",
                    SyntaxError,
                    self.idx,
                );
            }
        }

        if self.match_token(&Token::LeftParen) {
            self.open_parenthesis += 1;
            let expression = self.expression();
            if self.open_parenthesis > 0 {
                if self.match_token(&Token::RightParen) {
                    self.open_parenthesis -= 1;
                } else {
                    self.error("expected closing parenthesis", SyntaxError, self.idx);
                }
            }
            if expression == Expr::Null {
                self.error("not an expression", ParseError, start_idx);
            }
            return Expr::grouping(expression);
        }

        if is_literal_value_token(self.peek()) {
            return Expr::literal(self.advance_owned());
        }

        if self.peek() == &Token::RightParen && self.open_parenthesis == 0 {
            self.error(
                "closing parenthesis does not have a match",
                SyntaxError,
                self.idx,
            );
        }

        Expr::null()
    }

    // Help

    fn is_at_end(&self) -> bool {
        self.idx >= self.tokens.len()
    }

    fn match_token(&mut self, token: &Token) -> bool {
        while self.peek() == &Token::Space || self.peek() == &Token::Tab {
            self.advance();
        }
        if self.peek() == token {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.match_token(token) {
                return true;
            }
        }
        false
    }

    // Error handling

    fn error(&mut self, message: &str, kind: Error, idx: usize) {
        self.error = true;
        errors::scanner(message, kind, self.line, (idx + 1) as u32);
        self.synchronize();
    }

    fn synchronize(&mut self) {
        /*
        When there is an error, we need to get to a point where we can continue catching
        errors without being affected by the previous ones. That point is either in a literal value
        or a left prenthesis.
        */
        while !self.is_at_end() {
            if is_literal_value_token(self.peek()) {
                return;
            }
            if self.peek() == &Token::LeftParen {
                return;
            }
            if self.peek() == &Token::RightParen {
                if self.open_parenthesis > 0 {
                    self.open_parenthesis -= 1;
                } else {
                    self.advance();
                    self.error(
                        "closing parenthesis does not have a match",
                        SyntaxError,
                        self.idx,
                    );
                }
            }
            self.advance();
        }
    }

    // Token consuming

    fn previous(&self) -> &Token {
        if self.idx == 0 {
            return &Token::Null;
        }
        &self.tokens[self.idx - 1]
    }

    fn previous_owned(&self) -> Token {
        if self.idx == 0 {
            return Token::Null;
        }
        self.tokens[self.idx - 1].clone()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            return &Token::Null;
        }
        &self.tokens[self.idx]
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() { return &Token::Null }
        self.idx += 1;
        &self.tokens[self.idx - 1]
    }

    fn advance_owned(&mut self) -> Token {
        if self.is_at_end() { return Token::Null }
        self.idx += 1;
        self.tokens[self.idx - 1].clone()
    }
}
