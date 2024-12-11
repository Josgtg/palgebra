use std::fmt::{self, Display};

use crate::token::Token;

pub struct TokenSequence {
    tokens: Vec<Token>,
}

impl TokenSequence {
    pub fn new() -> Self {
        TokenSequence { tokens: Vec::new() }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        TokenSequence { tokens }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn pop_token(&mut self) {
        self.tokens.pop();
    }
}

impl Display for TokenSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for token in self.tokens.iter() {
            s.push(token.as_char());
        }
        f.write_str(&s)
    }
}
