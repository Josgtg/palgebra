#[derive(Clone, Debug, Eq, PartialEq, Hash)]

pub enum Token {
    And,
    Or,
    Not,
    IfOnlyIf,
    IfThen,
    LeftParen,
    RightParen,
    Sentence(char),
    Invalid,
    Null
}

impl Token {
    pub fn as_char(&self) -> char {
        match self {
            Token::And => '&',
            Token::Or => '|',
            Token::Not => '!',
            Token::IfOnlyIf => '~',
            Token::IfThen => '>',
            Token::LeftParen => '(',
            Token::RightParen => ')',
            Token::Sentence(c) => *c,
            Token::Invalid | Token::Null => '\0',
        }
    }
}