use std::fmt::{self, Display};

use crate::token::Token;

pub struct Proposition {
    tokens: Vec<Token>,
}

impl Proposition {
    pub fn new() -> Self {
        Proposition { tokens: Vec::new() }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Proposition { tokens }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

impl Display for Proposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for token in self.tokens.iter() {
            s.push(token.as_char());
        }
        f.write_str(&s)
    }
}
