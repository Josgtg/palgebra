use crate::{errors, grammar::Expr, token::Token};


pub fn parse(tokens: Vec<Token>) -> Result<Box<Expr>, ()> {
    let mut parser = Parser::new(tokens);
    parser.parse()

}


fn is_operator_token(token: &Token) -> bool {
    token == &Token::And || token == &Token::Or || token == &Token::IfOnlyIf || token == &Token::IfThen || token == &Token::Not
}


pub struct Parser {
    pub tokens: Vec<Token>,
    pub error: bool,
    open_parenthesis: u32,
    line: u32,
    col: u32,
    idx: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: Vec::new(),
            error: false,
            open_parenthesis: 0,
            line: 0,
            col: 0,
            idx: 0
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ()> {
        let proposition = self.proposition();

        while !self.is_at_end() {
            self.error = true;
            self.proposition();
        }

        if self.error { Err(()) }
        else { Ok(Box::new(proposition)) }
    }

    // Building the tree

    fn proposition(&mut self, )  -> Expr {
        let mut proposition = self.unary();
        
        let mut start_idx = self.col;

        while self.match_tokens(vec![Token::And, Token::Or, Token::IfOnlyIf, Token::IfThen]) {
            if proposition == Expr::Null {
                self.error("missing proposition on left side of operation", 0, start_idx);
                continue;
            }

            start_idx = self.col;
            
            let operator = self.previous_owned();

            let mut rigth = self.unary();

            if rigth == Expr::Null {
                if is_operator_token(self.peek()) {
                    self.error("operators are next to each other", 0, self.col);
                    continue;
                }

                if self.peek() == &Token::NewLine || self.peek() == &Token::Comment {
                    self.line += 1;
                    self.col = 1;
                    self.error("expected expression on right side of operator, but line finished", 0, self.col);
                }
                
                if self.peek() == &Token::RightParen {
                    if self.open_parenthesis > 0 {
                        self.open_parenthesis -= 1;
                    } else {
                        self.error("unmatched closing parenthesis", 0, self.col);
                        continue;
                    }
                }

                if self.match_token(Token::Invalid) {
                    self.error = true;
                    rigth = self.unary();
                } else {
                    self.error("missing proposition on right side of operation", 0, start_idx);
                    continue;
                }
            }

            proposition = Expr::Binary(Box::new(proposition), operator, Box::new(rigth))
        }

        if self.peek() == &Token::NewLine || self.peek() == &Token::Comment {
            self.line += 1;
            self.col = 1;
            proposition = self.proposition();
        }

        if let Token::Sentence(_) = self.peek() {
            self.error = true;
            proposition = self.proposition();
        }

        if self.match_token(Token::Invalid) || self.peek() == &Token::LeftParen {
            self.error = true;
            proposition = self.proposition();
        } 

        if self.peek() == &Token::Not {
            self.error("not operator is in an invalid position", 0, self.col);
        }

        proposition
    }

    fn unary(&mut self) -> Expr {
        let start_idx = self.col;

        if self.match_token(Token::Not) {
            let right = self.unary();
            if right == Expr::Null {
                self.error("missing proposition on right side of negation", 0, start_idx);
            }
            return Expr::Unary(Token::Not, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let start_idx = self.col;

        if let Token::Sentence(_) = self.previous() {
            if self.peek() == &Token::LeftParen {
                self.error("grouping in invalid position", 0, start_idx);
            }
            if let Token::Sentence(_) = self.peek() {
                self.error("simple proposition is in an invalid position", 0, self.col);
            }
        }

        if self.match_token(Token::LeftParen) {
            self.open_parenthesis += 1;
            let proposition = self.proposition();
            if self.open_parenthesis > 0 {
                if self.match_token(Token::RightParen) {
                    self.open_parenthesis -= 1;
                } else {
                    self.error("expected closing parenthesis", 0, self.col);
                }
            }   
            if proposition == Expr::Null {
                self.error("not a proposition", 1, start_idx);
            }
            return Expr::Grouping(Box::new(proposition))
        }

        if self.peek() == &Token::True || self.peek() == &Token::False {
            return Expr::Literal(self.advance_owned());
        }

        if let Token::Sentence(_) = self.peek() {
            return Expr::Literal(self.advance_owned());
        }

        if self.peek() == &Token::RightParen {
            if self.open_parenthesis == 0 {
                self.error("closing parenthesis does not have a match", 0, self.col);
            }
        }

        Expr::Null
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

    // Error handling

    fn error(&mut self, message: &str, code: u32, col: u32) {
        self.error = true;
        errors::report(message, code, self.line, col);
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
        if self.idx == 0 { return &Token::Null }
        &self.tokens[self.idx - 1]
    }

    fn previous_owned(&self) -> Token {
        if self.idx == 0 { return Token::Null }
        self.tokens[self.idx - 1].clone()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() { return &Token::Null }
        &self.tokens[self.idx]
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() { return &Token::Null }
        self.idx += 1;
        self.col += 1;
        &self.tokens[self.idx - 1]
    }

    fn advance_owned(&mut self) -> Token {
        if self.is_at_end() { return Token::Null }
        self.idx += 1;
        self.col += 1;
        self.tokens[self.idx - 1].clone()
    }
}
