use std::collections::HashSet;

use super::scanner;
use crate::token::Token;
pub struct Parser {
    pub sentences: HashSet<char>,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            sentences: HashSet::new(),
            tokens: Vec::new()
        }
    }

    pub fn scan(&mut self, proposition: String) {
       scanner::scan(self, proposition);
    }
}