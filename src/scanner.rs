use crate::token::Token;
use crate::errors::{self, Error::SyntaxError};

pub fn scan(proposition: &str, line: u32) -> Result<Vec<Token>, Vec<Token>> {
    let mut scanner = Scanner::new(line);
    let tokens = scanner.scan(proposition);
    if scanner.error {
        return Err(tokens)
    }
    Ok(tokens)
}


fn ignore(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\0' || c == '\n' || c == '\r'
}


struct Scanner {
    proposition: Vec<char>,
    tokens: Vec<Token>,
    error: bool,
    line: u32,
    col: u32,
    idx: usize
}

impl Scanner {
    fn new(line: u32) -> Self {
        Scanner {
            proposition: Vec::new(),
            tokens: Vec::new(),
            error: false,
            line,
            col: 1,
            idx: 0
        }
    }

    fn scan(&mut self, proposition: &str) -> Vec<Token> {
        self.proposition = proposition.chars().collect();
        while !self.is_at_end() {
            if self.match_new_line() { continue; }
            if ignore(self.peek()) {
                self.idx += 1;
                continue;
            }
            self.match_token();
        }

        if self.previous() == '\n' { self.tokens.pop(); }
        self.tokens.clone()
    }

    fn match_new_line(&mut self) -> bool {
        if self.peek() == '\n' {
            self.advance();
            self.tokens.push(Token::NewLine);
            self.line += 1;
            self.col = 1;
            return true;
        }
        false
    }

    fn match_token(&mut self) {
        match self.advance() {
            '&' => self.tokens.push(Token::And),
            '|' => self.tokens.push(Token::Or),
            '!' => self.tokens.push(Token::Not),
            '~' => self.tokens.push(Token::IfOnlyIf),
            '>' => self.tokens.push(Token::IfThen),
            '(' => self.tokens.push(Token::LeftParen),
            ')' => self.tokens.push(Token::RightParen),
            '/' => if self.match_char('/') {
                self.advance_until('\n');
                self.tokens.push(Token::Comment);
            }
            '1' => self.tokens.push(Token::True),
            't' => {
                if self.match_str("rue") { self.tokens.push(Token::True) }
                else { self.tokens.push(Token::Sentence('t')); }
            }
            '0' => self.tokens.push(Token::False),
            'f' => {
                if self.match_str("alse") { self.tokens.push(Token::False) }
                else { self.tokens.push(Token::Sentence('f')); }
            }
            _ => {
                if !self.previous().is_alphabetic() {
                    self.error = true;
                    errors::scanner(&format!("unexpected character \"{}\"", self.previous()), SyntaxError, self.line, self.col - 1);
                    self.tokens.push(Token::Invalid);
                    return
                }
                self.tokens.push(Token::Sentence(self.previous()));
            }
        }
    }

    fn previous(&self) -> char {
        if self.idx == 0 { return '\0' }
        self.proposition[self.idx - 1]
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.proposition[self.idx]
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() { return '\0' }
        self.idx += 1;
        self.col += 1;
        self.proposition[self.idx - 1]
    }

    fn match_char(&mut self, to_match: char) -> bool {
        if self.peek() == to_match {
            self.advance();
            return true
        }
        false
    }

    fn match_str(&mut self, to_match: &str) -> bool {
        let backup = self.idx;
        for c in to_match.chars() {
            if self.peek() == c { self.advance(); }
            else {
                self.idx = backup;
                return false
            }
        }
        true
    }

    fn advance_until(&mut self, to_stop: char) {
        while !self.is_at_end() && !self.match_char(to_stop) { self.advance(); }
    }

    fn is_at_end(&self) -> bool {
        self.idx >= self.proposition.len()
    }
}
